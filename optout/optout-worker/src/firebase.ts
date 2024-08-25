import { FirebaseRequestFailureError, FirebaseAuthFailureError } from './errors';
import * as Firestore from 'fireworkers';

async function getFirestoreDB(env: Env): Promise<Firestore.DB> {
    const serviceAccountJSON = JSON.parse(env.FIREBASE_CREDS)
    const db = await Firestore.init(serviceAccountJSON)
    return db
}

export async function addOptOut(userId: string, platform: string, env: Env) {
    const db = await getFirestoreDB(env)
    const doc = await Firestore.set(db, 'opt_outs', `${userId}_${platform}`, { id: userId, platform })
}

export async function removeOptOut(userId: string, platform: string, env: Env) {
    const db = await getFirestoreDB(env)
    const doc = await Firestore.remove(db, 'opt_outs', `${userId}_${platform}`)
}
