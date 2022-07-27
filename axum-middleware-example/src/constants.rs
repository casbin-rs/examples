// Messages
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_CAN_NOT_FIND_USER: &str = "Can not find user, user not exist";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "Can not insert data";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
pub const MESSAGE_SIGNIN_FAILED: &str = "Wrong email or password, please try again";
pub const MESSAGE_SIGNIN_SUCCESS: &str = "Signin successfully";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";

// BAD REQUEST
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

// HEADERS
pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// IGNORE ROUTES
pub const IGNORE_ROUTES: [&str; 4] = [
    "api/auth/register",
    "api/auth/signin",
    "api/users",
    "api/user",
];
