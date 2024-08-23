import { getTokenFromGCPServiceAccount } from '@sagi.io/workers-jwt';
import { FirebaseRequestFailureError, FirebaseAuthFailureError } from './errors';

const FIREBASE_BASE_URL = 'https://firestore.googleapis.com/v1';

/**
 * Retrieves a JWT signed access token for Firebase REST API requests.
 *
 * @param env - The worker's environment variables.
 * @returns The access token.
 */
export async function getAccessToken(env: Env) {
    const jwtToken = await getTokenFromGCPServiceAccount({
        serviceAccountJSON: JSON.parse(env.FIREBASE_CREDS),
        aud: 'https://oauth2.googleapis.com/token',
        payloadAdditions: {
            scope: ['https://www.googleapis.com/auth/datastore'].join(' '),
        },
    });

    const accessToken = await (
        await fetch('https://oauth2.googleapis.com/token', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: new URLSearchParams({
                grant_type: 'urn:ietf:params:oauth:grant-type:jwt-bearer',
                assertion: jwtToken,
            }),
        })
    ).json();

    return accessToken;
}

/**
 * Shortcut for performing a request from the specified URL with Firebase authentication.
 *
 * @param url - The URL to fetch data from.
 * @param options - A RequestInit with options for the fetch request.
 * @param env - The worker's environment variables.
 * @returns A promise that resolves to the response from the fetch request.
 */
async function fetchWithAuth(url: string, options: RequestInit, env: Env): Promise<Response> {
    const accessToken = await getAccessToken(env);
    const headers = {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${accessToken.access_token}`,
    };
    return fetch(url, { ...options, headers });
}

/**
 * Checks if a document (an optout) exists in the Firestore.
 *
 * @param userId - The user ID.
 * @param platform - The platform.
 * @param env - The worker's environment variables.
 * @returns A promise that resolves to a boolean indicating if the document exists.
 */
async function documentExists(userId: string, platform: string, env: Env): Promise<boolean> {
    const queryData = {
        structuredQuery: {
            where: {
                compositeFilter: {
                    op: 'AND',
                    filters: [
                        { fieldFilter: { field: { fieldPath: 'id' }, op: 'EQUAL', value: { stringValue: userId } } },
                        {
                            fieldFilter: {
                                field: { fieldPath: 'platform' },
                                op: 'EQUAL',
                                value: { stringValue: platform },
                            },
                        },
                    ],
                },
            },
            from: [{ collectionId: 'opt_outs' }],
            select: { fields: [{ fieldPath: '__name__' }] },
        },
    };

    const url = `${FIREBASE_BASE_URL}/projects/${env.FIREBASE_PROJECT_ID}/databases/(default)/documents:runQuery`;
    const response = await fetchWithAuth(url, { method: 'POST', body: JSON.stringify(queryData) }, env);

    if (response.status === 401 || response.status === 403) {
        throw new FirebaseAuthFailureError('Firebase auth failed', await response.json());
    } else if (!response.ok) {
        throw new FirebaseRequestFailureError('Could not check if document exists', await response.json());
    }

    const results = await response.json();
    return results.length > 0 && results[0].document;
}

export async function addOptOut(userId: string, platform: string, env: Env) {
    if (await documentExists(userId, platform, env)) {
        return { success: true };
    }

    const url = `${FIREBASE_BASE_URL}/projects/${env.FIREBASE_PROJECT_ID}/databases/(default)/documents/opt_outs/`;
    const documentData = { fields: { id: { stringValue: userId }, platform: { stringValue: platform } } };
    const response = await fetchWithAuth(url, { method: 'POST', body: JSON.stringify(documentData) }, env);

    if (response.status === 401 || response.status === 403) {
        throw new FirebaseAuthFailureError('Firebase auth failed', await response.json());
    } else if (!response.ok) {
        throw new FirebaseRequestFailureError('Could not add document', await response.json());
    }
}

export async function removeOptOut(userId: string, platform: string, env: Env) {
    console.log(`Opting in ${userId} on platform ${platform}`);

    const queryData = {
        structuredQuery: {
            where: {
                compositeFilter: {
                    op: 'AND',
                    filters: [
                        { fieldFilter: { field: { fieldPath: 'id' }, op: 'EQUAL', value: { stringValue: userId } } },
                        {
                            fieldFilter: {
                                field: { fieldPath: 'platform' },
                                op: 'EQUAL',
                                value: { stringValue: platform },
                            },
                        },
                    ],
                },
            },
            from: [{ collectionId: 'opt_outs' }],
            select: { fields: [{ fieldPath: '__name__' }] },
        },
    };

    const url = `${FIREBASE_BASE_URL}/projects/${env.FIREBASE_PROJECT_ID}/databases/(default)/documents:runQuery`;
    const response = await fetchWithAuth(url, { method: 'POST', body: JSON.stringify(queryData) }, env);
    const data = await response.json();

    if (response.status === 401 || response.status === 403) {
        throw new FirebaseAuthFailureError('Firebase auth failed', await response.json());
    } else if (!response.ok) {
        throw new FirebaseRequestFailureError('Could query documents', await response.json());
    }

    for (const doc of data) {
        const deleteUrl = `${FIREBASE_BASE_URL}/${doc.document.name}`;
        const deleteResponse = await fetchWithAuth(deleteUrl, { method: 'DELETE' }, env);

        if (deleteResponse.status === 401 || response.status === 403) {
            throw new FirebaseAuthFailureError('Firebase auth failed', await response.json());
        } else if (!deleteResponse.ok) {
            throw new FirebaseRequestFailureError('Could not delete document', await response.json());
        }
    }
    return { success: true };
}
