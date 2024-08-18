type CommandResponse = CommandSucess | InvalidSignature | UnknownCommand | FirebaseFailure;

type CommandSucess = {
    success: true
}

type Failure = {
    success: false,
};

type InvalidSignature = Failure & {
    reason: "signature was not valid",
};

type UnknownCommand = Failure & {
    reason: "Unknown command",
};