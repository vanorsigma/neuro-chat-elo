import { getTokenFromGCPServiceAccount } from '@sagi.io/workers-jwt'

export async function getAccessToken(env: Env) {
    const jwtToken = await getTokenFromGCPServiceAccount({
        serviceAccountJSON: JSON.parse(env.FIREBASE_CREDS),
        aud: 'https://oauth2.googleapis.com/token',
        payloadAdditions: {
            scope: [
                'https://www.googleapis.com/auth/datastore',
            ].join(' '),
        },
    })
    
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
    ).json()
    
    return accessToken
}

async function fetchWithAuth(url: string, options: RequestInit, env: Env): Promise<Response> {
    const accessToken = await getAccessToken(env);
    const headers = {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${accessToken.access_token}`,
    };
    return fetch(url, { ...options, headers });
}

async function documentExists(userId: string, platform: string, env: Env): Promise<boolean> {
    const queryData = {
        structuredQuery: {
            where: {
                compositeFilter: {
                    op: "AND",
                    filters: [
                        { fieldFilter: { field: { fieldPath: "id" }, op: "EQUAL", value: { stringValue: userId } } },
                        { fieldFilter: { field: { fieldPath: "platform" }, op: "EQUAL", value: { stringValue: platform } } }
                    ]
                }
            },
            from: [{ collectionId: "opt_outs" }],
            select: { fields: [{ fieldPath: "__name__" }] },
        }
    };
    
    const url = `${env.FIREBASE_BASE_URL}/projects/${env.FIREBASE_PROJECT_ID}/databases/(default)/documents:runQuery`;
    const response = await fetchWithAuth(url, { method: 'POST', body: JSON.stringify(queryData) }, env);
    
    if (!response.ok) {
        const errorDetails = await response.json();
        throw new Error(`Failed to run query: ${response.statusText} - ${JSON.stringify(errorDetails)}`);
    }
    
    const results = await response.json();
    return results.length > 0 && results[0].document;
}

export async function addOptOut(userId: string, platform: string, env: Env): Promise<CommandResponse> {
    if (await documentExists(userId, platform, env)) {
        return { success: true };
    }
    
    const url = `${env.FIREBASE_BASE_URL}/projects/${env.FIREBASE_PROJECT_ID}/databases/(default)/documents/opt_outs/`;
    const documentData = { fields: { id: { stringValue: userId }, platform: { stringValue: platform } } };
    const response = await fetchWithAuth(url, { method: 'POST', body: JSON.stringify(documentData) }, env);
    
    return response.ok ? { success: true } : { success: false, reason: 'Firebase request failed' };
}

export async function removeOptOut(userId: string, platform: string, env: Env): Promise<CommandResponse> {
    console.log(`Opting in ${userId} on platform ${platform}`);
    
    const queryData = {
        structuredQuery: {
            where: {
                compositeFilter: {
                    op: "AND",
                    filters: [
                        { fieldFilter: { field: { fieldPath: "id" }, op: "EQUAL", value: { stringValue: userId } } },
                        { fieldFilter: { field: { fieldPath: "platform" }, op: "EQUAL", value: { stringValue: platform } } }
                    ]
                }
            },
            from: [{ collectionId: "opt_outs" }],
            select: { fields: [{ fieldPath: "__name__" }] },
        }
    };
    
    const url = `${env.FIREBASE_BASE_URL}/projects/${env.FIREBASE_PROJECT_ID}/databases/(default)/documents:runQuery`;
    const response = await fetchWithAuth(url, { method: 'POST', body: JSON.stringify(queryData) }, env);
    const data = await response.json();
    
    for (const doc of data) {
        const deleteUrl = `${env.FIREBASE_BASE_URL}/${doc.document.name}`;
        const deleteResponse = await fetchWithAuth(deleteUrl, { method: 'DELETE' }, env);
        
        if (!deleteResponse.ok) {
            console.error(`Failed to delete document: ${deleteResponse.statusText}`);
            return { success: false, reason: 'Firebase request failed' };
        }
    }
    return { success: true };
}