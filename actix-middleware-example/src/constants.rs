// Messages
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_CAN_NOT_FIND_USER: &str = "Can not find user, user not exist";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "Can not insert data";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_LOGOUT_FAILED: &str = "Logout fail";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
pub const MESSAGE_DELETE_USER_ERROR: &str = "Can not delete user, please try again";
pub const MESSAGE_DELETE_USER_PERMISSION_ERROR: &str =
    "Can not delete user due to permission control";
pub const MESSAGE_DELETE_POST_PERMISSION_ERROR: &str =
    "Can not delete post due to permission control";
pub const MESSAGE_DELETE_USER_SUCCESS: &str = "Delete user successful";
pub const MESSAGE_NEW_USER_ADD_PERMISSION_ERROR: &str =
    "Can not add new user when adding new permissions";
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

// Headers
pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 3] =
    ["/api/auth/signup", "/api/auth/login", "/api/posts"];
