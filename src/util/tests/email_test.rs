use util::email::VerifyEmailClaims;
use util::jwt::{create_jwt_token, verify_token};
use util::util::create_exp;

const PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAu7GM38CWWJUdVtZzjukVN2VdIwTOiP1YMadzc3wI5K8cT98O
iDcGw662/Y9JYwKZX4Vjgu7HNyoLQPstUMGuqZ8eIYkUM10Ny8dMoQz9jwbC2bXW
sXw/+homUbs9uApYZfnvOTcc2QaABsNBpgikA4RqdpFxkVx5SkCrfxWYR8T8np15
H02a8elV1Y0bK43G8Lp8/J2boKLuxLEkkt/7i/TE3LH9tfbMonnxQJ6ah0GJ0PFm
M5zRC+WJFlyffdv0NgVNtEcGoioeTcidjspPVc5QUEf9rKd1vIUwmAdY+oBzJqVU
2ppFxqms1qNzPtB3wlkCLKyWq+pOP/8o3Q7lEQIDAQABAoIBAEIca+50O4DAJTFt
IA76coqsnX20N7tw4Ru8S6Il0kwakfJgMxIvr6ZFaBcv3T2OsM9AgKihynHX0Vb2
IiQL1KXSch6F1iOxZyab8JXZqYYzb3aX4iNqDG7fvxeSyE6G9lQVEPoqO3MGIxo+
tGTzLjI+9fEFkecNl3tcBAwjYxwD34nqE4wUWpnnL5q3q5blsjv1SwcgP/8HPUUX
WSIL20+QANL7x9OLycH8axuacy/c7M3ZSLiTO1xMOcudnxTgJC7q3tQiZdULFkyJ
TlQxEnXmOetj0pooGjHnEvy3ecSiRh7XXIy5r7/LrygTXGa6noXbFZ41U9ad3LS3
k9/iwwECgYEA8P4vDiDLp9gswxkVanQCBgKph32o98GwvRzhoM882PoByAcpDdE2
QkI47bdQacRnBK07Oef/3wxoW/pqspP1QmypyNq/is81AmaRKkRHN5qSaesoR5IG
GAC34oRtFZcqK4e9GW31SzJwF4TxZUgtIVNgun09Z4yMafMljEHYg6kCgYEAx2Gw
Uf7ob2pzla/r+dZzTIL8sqQjGDCqSJX4nIu7pqKuLnCFZsj8uOPBpRggYVqQCF6C
a8xBvg7XjuE4HfKmDPTL0YOt91Pmp69T5cRUnl/jwltjyAw6UIlZ06Vivd0xV4iT
6TmPxQU3J7V6HJQ28/MZguQlNrVquCI3mnuktykCgYEAuM0yqFLd2Ugciz+g/JEZ
PCKufggOmm8daOfM3dQNhRWyaAGVqdLVGNpPbXs1XuYNWbi9whn2TMTnAiF10JtF
aDqLuEqGt5tNuxmMGC1jr3AI9tAtgzSnF6D1Ye7K99ODsv3UNLvtbkyvSFnaAyDl
+rhFZLzFabKNunGKSZP0xIECgYB1OZZ1Xy5B85CpvZk8dz2rCoiQb7jCjDpSGvrk
77MQsi0+2KFWYW1VN3OvIZRIKPruMML1pk4w9IEAXKKl2PiQPY1oTceG1ymSBZ+K
Q/iMdUxwBVORVr8igt7xdRetEdPk50Qvp8xDxmE7U5PMIch9fbzOHF1U4S0xvcx7
Y57goQKBgQCcV/iqPE/aLHdhjkVVfnru8yEzhOrGdMZ/S1wDPlRK4z/knOxXY45W
zWCbL713fo8oBG78RvrGQOgypxqY7a/oKlMzwdsgZFF2pIDi4X6u0W0wCA70AV6P
UuovMB0XVn5QV7s82DLYoP67OUF5r8WOtiR2D78B+dvqb7bX5DUw8Q==
-----END RSA PRIVATE KEY-----
";
const PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAu7GM38CWWJUdVtZzjukV
N2VdIwTOiP1YMadzc3wI5K8cT98OiDcGw662/Y9JYwKZX4Vjgu7HNyoLQPstUMGu
qZ8eIYkUM10Ny8dMoQz9jwbC2bXWsXw/+homUbs9uApYZfnvOTcc2QaABsNBpgik
A4RqdpFxkVx5SkCrfxWYR8T8np15H02a8elV1Y0bK43G8Lp8/J2boKLuxLEkkt/7
i/TE3LH9tfbMonnxQJ6ah0GJ0PFmM5zRC+WJFlyffdv0NgVNtEcGoioeTcidjspP
Vc5QUEf9rKd1vIUwmAdY+oBzJqVU2ppFxqms1qNzPtB3wlkCLKyWq+pOP/8o3Q7l
EQIDAQAB
-----END PUBLIC KEY-----
";

#[test]
fn verify_email_test() {
    let code = create_jwt_token(
        PRIVATE_KEY.as_bytes(),
        VerifyEmailClaims {
            exp: create_exp(60 * 5),
            email: "123".to_string(),
        },
    )
    .unwrap();

    let data = verify_token::<VerifyEmailClaims>(code, PUBLIC_KEY.as_bytes())
        .unwrap()
        .claims;

    assert_eq!(data.email.as_str(), "123");
}
