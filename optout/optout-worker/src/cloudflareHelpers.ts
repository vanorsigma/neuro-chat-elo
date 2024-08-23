export async function updateSecret(secretName: string, secretValue: string, env: Env): Promise<void> {
    const url = `https://api.cloudflare.com/client/v4/accounts/${env.CLOUDFLARE_ACCOUNT_ID}/workers/scripts/${env.CLOUDFLARE_WORKER_NAME}/secrets`;
    const body = JSON.stringify({
        name: secretName,
        text: secretValue,
        type: 'secret_text',
    });

    const response = await fetch(url, {
        method: 'PUT',
        headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${env.CLOUDFLARE_API_KEY}`,
        },
        body: body,
    });

    if (!response.ok) {
        throw new Error(`Failed to update secret: ${response.status} - ${response.statusText}`);
    }
}
