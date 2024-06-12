pub enum AuthApiError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
    MissingToken,
    InvalidToken,
    IncorrectCredentials
}