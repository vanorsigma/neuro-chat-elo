type ErrorName =
    | 'InvalidSignature'
    | 'UnknownCommand'
    | 'FirebaseRequestFailure'
    | 'FirebaseAuthFailure'
    | 'TwitchRequestFailure'
    | 'TwitchAuthFailure';

export class BaseError<T extends ErrorName> extends Error {
    name: T;
    message: string;
    cause: any;

    constructor(name: T, message: string, cause: any) {
        super(message);
        this.name = name;
        this.message = message;
        this.cause = cause;
    }
}

export class InvalidSignatureError extends BaseError<'InvalidSignature'> {
    constructor(message: string, cause: any) {
        super('InvalidSignature', message, cause);
    }
}
export class UnknownCommandError extends BaseError<'UnknownCommand'> {
    constructor(message: string, cause: any) {
        super('UnknownCommand', message, cause);
    }
}

export class FirebaseRequestFailureError extends BaseError<'FirebaseRequestFailure'> {
    constructor(message: string, cause: any) {
        super('FirebaseRequestFailure', message, cause);
    }
}
export class FirebaseAuthFailureError extends BaseError<'FirebaseAuthFailure'> {
    constructor(message: string, cause: any) {
        super('FirebaseAuthFailure', message, cause);
    }
}

export class TwitchRequestFailureError extends BaseError<'TwitchRequestFailure'> {
    constructor(message: string, cause: any) {
        super('TwitchRequestFailure', message, cause);
    }
}
export class TwitchAuthFailureError extends BaseError<'TwitchAuthFailure'> {
    constructor(message: string, cause: any) {
        super('TwitchAuthFailure', message, cause);
    }
}
