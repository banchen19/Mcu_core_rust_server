// 邮箱验证码管理

use std::{collections::HashMap, time::SystemTime};

use actix::{Actor, Context, Handler, Message};
use lettre::message::Mailbox;
use lettre::Message as LettreMessage;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, SmtpTransport,
    Transport,
};

use build_html::*;

#[derive(Debug, Clone)]
pub struct EmaiCodeManager {
    //验证码、邮箱、过期时间
    codelist: HashMap<String, (String, SystemTime)>,
}

impl EmaiCodeManager {
    pub fn new() -> Self {
        Self {
            codelist: HashMap::new(),
        }
    }
    // 生成验证码
    pub fn generate_code(&mut self, email: String) -> String {
        let code = format!("{:06}", rand::random::<u32>() % 1000000);
        self.codelist
            .insert(email.clone(), (code.clone(), SystemTime::now()));
        code
    }

    // 验证验证码
    pub fn verify_code(&mut self, email: String, code: String) -> bool {
        match self.codelist.get(&email) {
            Some((c, _)) => {
                let now = SystemTime::now();
                let duration = now
                    .duration_since(self.codelist.get(&email).unwrap().1)
                    .unwrap();
                // 验证时间是否超过5分钟
                if c == &code && duration.as_secs() < 300 {
                    self.codelist.remove(&email);
                    return true;
                } else {
                    return false;
                }
            }
            None => false,
        }
    }
}
impl Actor for EmaiCodeManager {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "String")]
pub struct GenerateCode {
    pub email: String,
}

impl Handler<GenerateCode> for EmaiCodeManager {
    type Result = String;

    fn handle(&mut self, generatecode: GenerateCode, _: &mut Context<Self>) -> String {
        self.generate_code(generatecode.email)
    }
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct VerifyCode {
    pub email: String,
    pub code: String,
}

impl Handler<VerifyCode> for EmaiCodeManager {
    type Result = bool;

    fn handle(&mut self, verifycode: VerifyCode, _: &mut Context<Self>) -> bool {
        self.verify_code(verifycode.email, verifycode.code)
    }
}

#[derive(Debug, Clone)]
pub struct EmailManager {
    smtp_ip: String,
    server_name: String,
    credentials: Credentials,
    from_email: String,
}

impl EmailManager {
    pub fn new(server_name: String, smtp_ip: String, username: String, password: String) -> Self {
        // 检查server_name、smtp_ip、username、password某个是否为空，如果为空，中断程序
        if server_name.is_empty() || smtp_ip.is_empty() || username.is_empty() || password.is_empty() {
            panic!("server_name、smtp_ip、username、password均不能为空");
        }
        let credentials = Credentials::new(username.clone(), password);
        Self {
            smtp_ip,
            server_name,
            credentials,
            from_email: username,
        }
    }

    pub fn send(
        self,
        msg: LettreMessage,
    ) -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
        let mailer = SmtpTransport::relay(&self.smtp_ip)
            .unwrap()
            .credentials(self.credentials)
            .build();
        mailer.send(&msg)
    }

    // 生成验证码的html
    fn generate_code_html(self, code: String) -> String {
        let msg = format!(
            r#"<table width="500" border="0" align="center" cellpadding="0" cellspacing="0">
    <div class="inner-div">
        <table width="100%" border="0" cellspacing="0" cellpadding="0">
            <td class="dynamic-machine-td" valign="middle">
            {}
            </td>
            <body>
                <tr class="spacer-tr">
                    <td class="email-verification-code-td">
                        邮箱验证码
                    </td>
                </tr>
                <tr>
                    <td class="user-greeting-td">

                    <br>
                        亲爱的玩家你好！
</br>
                        你的验证码是：{},请在 5 分钟内进行验证。如果该验证码不为您本人申请，请无视。
                    </td>
                </tr>
                <tr class="spacer-tr"></tr>
            </body>
        </table>
    </div>
</table>"#,
            self.server_name, code
        );

        HtmlPage::new()
            .with_style(
                r#".outer-div {
        background: #eee;
    }
    .inner-div {
        background: #fff;
    }
    .dynamic-machine-td {
        padding-left: 30px;
        background-color: #415a94;
        color: #fff;
        padding: 20px 40px;
        font-size: 21px;
    }
    .email-verification-code-td {
        font-size: 24px;
        line-height: 1.5;
        color: #000;
        margin-top: 40px;
    }
    .user-greeting-td {
        font-size: 14px;
        color: #333;
        padding: 24px 40px 0 40px;
    }
    .spacer-tr {
        padding: 40px;
        display: table-cell;
    }"#,
            )
            .with_container(
                Container::new(ContainerType::Div)
                    .with_attributes([("class", "outer-div")])
                    .with_raw(msg),
            )
            .to_html_string()
    }

    pub fn send_email_code(
        self,
        code: String,
        to_email: Mailbox,
    ) -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
        let msg = self.clone().generate_code_html(code.clone());
        let email = LettreMessage::builder()
            .from(self.from_email.parse().unwrap()) //发送者
            .to(to_email) //接收者
            .subject(self.server_name + "邮箱验证码")
            .header(ContentType::TEXT_HTML)
            .body(String::from(msg)) //邮箱内容
            .unwrap();

        let mailer = SmtpTransport::relay(&self.smtp_ip)
            .unwrap()
            .credentials(self.credentials)
            .build();
        mailer.send(&email)
    }
}

impl Actor for EmailManager {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EmailSend {
    pub msg: LettreMessage,
}

impl Handler<EmailSend> for EmailManager {
    type Result = ();

    fn handle(&mut self, emailsend: EmailSend, _: &mut Context<Self>) {
        let _ = self.clone().send(emailsend.msg);
    }
}

#[derive(Message)]
#[rtype(
    result = "Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error>"
)]
pub struct EmailCodeSend {
    pub code: String,
    pub to_email: Mailbox,
}

impl Handler<EmailCodeSend> for EmailManager {
    type Result =
        Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error>;

    fn handle(
        &mut self,
        emailcodesend: EmailCodeSend,
        _: &mut Context<Self>,
    ) -> Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
        self.clone()
            .send_email_code(emailcodesend.code, emailcodesend.to_email)
    }
}
