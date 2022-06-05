var searchIndex = JSON.parse('{\
"router":{"doc":"","t":[0,0,0,0,0,0,0,0,0,0,5,5,5,5,5,0,5,5,5,5,5,0,0,0,0,6,3,3,3,3,3,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,18,3,18,18,18,18,18,18,18,18,11,11,12,11,11,14,11,11,11,12,11,11,11,11,11,3,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,3,11,11,12,12,11,11,11,11,12,11,11,11,11,12,11,5],"n":["apis","catch","data","resource","authentication","user","verify_email","api","data","util","facebook_oauth","facebook_oauth_code","google_oauth","google_oauth_code","connect_account","api","login","sign_up","user_info","verify_email","not_found","auth_data","code","response","user","AuthError","AuthUrl","Claims","LoginFromData","SignUp","Token","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","deserialize","email","email","exp","finalize","finalize","fmt","from","from","from","from","from","id","init","init","into","into","into","into","into","into_collection","into_collection","into_collection","into_collection","into_collection","mapped","mapped","mapped","mapped","mapped","modes","modes","password","password","push_data","push_data","push_value","push_value","serialize","serialize","serialize","token","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","url","username","username","verified_email","vzip","vzip","vzip","vzip","vzip","AuthError","Code","LoginPasswordError","LoginUserNotFoundError","NotFound","OAuthCodeError","OAuthGetUserInfoError","Ok","SignUpEmailAlreadyRegistered","VerifyEmailError","borrow","borrow_mut","code","fmt","from","generate_code","into","into_collection","mapped","message","serialize","try_from","try_into","type_id","vzip","Response","borrow","borrow_mut","code","data","fmt","from","into","into_collection","mapped","new","serialize","try_from","try_into","type_id","vzip","UserInfo","borrow","borrow_mut","connects","email","from","into","into_collection","mapped","modes","serialize","try_from","try_into","type_id","username","vzip","hello_world"],"q":["router","","","","router::apis","","","router::apis::authentication","","","router::apis::authentication::api","","","","router::apis::authentication::util","router::apis::user","router::apis::user::api","","","router::apis::verify_email","router::catch","router::data","","","","router::data::auth_data","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","router::data::code","","","","","","","","","","","","","","","","","","","","","","","","","router::data::response","","","","","","","","","","","","","","","","router::data::user","","","","","","","","","","","","","","","","router::resource"],"d":["","","","","Authenticate APIs","","","","","","Get Facebook OAuth url","Facebook OAuth2 login","Get Google OAuth url","Google OAuth2 login","","","User login API","Sign up account API","Get login user info","","","","","","","","","","","","","","","","","","","","","","","","","","Required (validate_exp defaults to true in validation). …","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","This token is invalid.","","Input password error.","User not found error.","Resource not found.","OAuth auth code error.","OAuth get user info error.","Ok.","This email is already registered.","This code is invalid.","","","","","Returns the argument unchanged.","Example","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,3,4,5,1,2,3,4,5,5,3,4,5,3,4,5,1,2,3,4,5,5,3,4,1,2,3,4,5,1,2,3,4,5,1,2,3,4,5,4,5,3,4,3,4,3,4,1,2,5,2,1,2,3,4,5,1,2,3,4,5,1,2,3,4,5,1,4,5,5,1,2,3,4,5,6,0,6,6,6,6,6,6,6,6,6,6,6,6,6,0,6,6,6,6,6,6,6,6,6,0,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,0,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,0],"f":[null,null,null,null,null,null,null,null,null,null,[[["string",3],["state",3]],["json",3,[["response",3,[["authurl",3]]]]]],[[["string",3],["string",3],["state",3],["state",3],["requestip",3]]],[[["string",3],["state",3]],["json",3,[["response",3,[["authurl",3]]]]]],[[["string",3],["string",3],["state",3],["state",3],["requestip",3]]],[[["oauthdata",3],["string",3],["state",3],["string",3],["requestip",3]]],null,[[["form",3,[["loginfromdata",3]]],["state",3],["state",3]]],[[["form",3,[["signup",3]]],["state",3],["state",3],["requestip",3]]],[[["result",4,[["loginuserdata",3],["unauthorized",3,[["json",3,[["response",3,[["string",3]]]]]]]]],["state",3]]],[[["string",3],["state",3],["state",3]]],[[["request",3]],["json",3,[["response",3,[["option",4,[["string",3]]]]]]]],null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[],["result",4]],null,null,null,[[],["result",4,[["errors",3]]]],[[],["result",4,[["errors",3]]]],[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],null,[[["options",3]]],[[["options",3]]],[[]],[[]],[[]],[[]],[[]],[[],["smallvec",3]],[[],["smallvec",3]],[[],["smallvec",3]],[[],["smallvec",3]],[[],["smallvec",3]],[[],["smallvec",3]],[[],["smallvec",3]],[[],["smallvec",3]],[[],["smallvec",3]],[[],["smallvec",3]],null,null,null,null,[[["fromformgeneratedcontext",3],["datafield",3]],["pin",3,[["box",3,[["future",8]]]]]],[[["fromformgeneratedcontext",3],["datafield",3]],["pin",3,[["box",3,[["future",8]]]]]],[[["valuefield",3]]],[[["valuefield",3]]],[[["",0]],["result",4]],[[["",0]],["result",4]],[[["",0]],["result",4]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,null,[[]],[[]],[[]],[[]],[[]],null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],null,[[["",0],["formatter",3]],["result",6]],[[]],null,[[]],[[],["smallvec",3]],[[],["smallvec",3]],null,[[["",0]],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[]],null,[[["",0]],["",0]],[[["",0]],["",0]],null,null,[[["",0],["formatter",3]],["result",6]],[[]],[[]],[[],["smallvec",3]],[[],["smallvec",3]],[[["code",3],["option",4]],["json",3,[["response",3]]]],[[["",0]],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[]],null,[[["",0]],["",0]],[[["",0]],["",0]],null,null,[[]],[[]],[[],["smallvec",3]],[[],["smallvec",3]],null,[[["",0]],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],null,[[]],[[],["json",3,[["response",3,[["string",3]]]]]]],"p":[[3,"AuthUrl"],[3,"Token"],[3,"LoginFromData"],[3,"SignUp"],[3,"Claims"],[3,"Code"],[3,"Response"],[3,"UserInfo"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};