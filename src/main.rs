use std::rc::Rc;

use provider::{AccountViewData, Provider};
use yew::prelude::*;
use yewtil::future::LinkFuture;

mod dependencies;
mod provider;

enum Msg {
    GetAddressInfos,
    ShowAccount(AccountViewData),
    ShowError(String),
    UpdateAddressInput(String),
}

struct Model {
    link: ComponentLink<Self>,
    provider: Rc<Provider>,
    account: Option<AccountViewData>,
    error_msg: String,
    address_input: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let algod = dependencies::algod().expect("Couldn't initialize algod");
        let provider = dependencies::provider(algod);
        Self {
            link,
            provider: Rc::new(provider),
            account: None,
            error_msg: "".to_owned(),
            address_input: "".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetAddressInfos => {
                let provider = self.provider.clone();
                let address_res = self.address_input.parse();
                match address_res {
                    Ok(address) => self.link.send_future(async move {
                        match provider.get_infos(&address).await {
                            Ok(metrics) => Msg::ShowAccount(metrics),
                            Err(err) => Msg::ShowError(format!("{}", err)),
                        }
                    }),
                    Err(e) => self.link.send_message(Msg::ShowError(format!("{}", e))),
                }
                false
            }
            Msg::ShowAccount(account) => {
                self.error_msg = "".to_owned();
                self.account = Some(account);
                true
            }
            Msg::ShowError(msg) => {
                self.error_msg = msg;
                true
            }
            Msg::UpdateAddressInput(input) => {
                self.address_input = input;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <input
                placeholder="Address"
                size=64
                value=self.address_input.clone()
                oninput=self.link.callback(|e: InputData| Msg::UpdateAddressInput(e.value))
            />
            <button onclick=self.link.callback(|_| Msg::GetAddressInfos)>{ "Get infos" }</button>

            <p style="color: red;"> { self.error_msg.clone() }</p>
            <div>{ self.account_info_container() } </div>
        </div>
        }
    }
}

impl Model {
    fn account_info_container(&self) -> Html {
        match &self.account {
            Some(account) => self.account_info_table(account),
            None => Self::no_account(),
        }
    }

    fn no_account() -> Html {
        html! {
            <div/>
        }
    }

    fn account_info_table(&self, account: &AccountViewData) -> Html {
        html! {
            <dl>
            // <dt>{"Address:"}</dt>
            // <dd>{view_data.address.clone()}</dd>
            <dt>{"Status:"}</dt>
            <dd>{account.status.clone()}</dd>
            <dt>{"Holdings:"}</dt>
            <dd>{account.holdings.clone()}</dd>
            <dt>{"Rewards:"}</dt>
            <dd>{account.rewards.clone()}</dd>
            <dt>{"Pending rewards:"}</dt>
            <dd>{account.pending_rewards.clone()}</dd>
            </dl>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
