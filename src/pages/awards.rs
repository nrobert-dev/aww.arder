use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use serde::{Deserialize, Serialize};
use yew::format::Json;
use yew::services::storage::Area;
use yew::services::StorageService;


pub const KEY: &'static str = "yew.aww.ards.database";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    members : Vec<String>
}
impl Database {
    pub fn new() -> Self {
        Database { members : Vec::new()}
    }
}

struct AwardOptions {
    from : String,
    to : String,
    award : String
}

struct OfficialAward {
    from : String,
    to : String,
    award : String,
    message : String
}

pub struct Awards {
    link : ComponentLink<Self>,
    props : Props,
    members: Vec<String>,
    options : AwardOptions,
    message : String,
    awards : Vec<OfficialAward>
}

#[derive(Clone,Debug,Properties)]
pub struct Props {
    pub team_name : String,
}

pub enum Msg {
    OnMessageChange(String),
    OnOptionChange(String, String),
    AddAward
}

impl Component for Awards {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let Json(database) = storage.restore(KEY);
        let database = database.unwrap_or_else(|_| Database::new());
        let members = database.members;

        Self {
            link,
            props,
            options : AwardOptions {
                from : String::from(&members[0].clone()),
                to : String::from(&members[1].clone()),
                award : String::from("🌼"),
            },
            members,
            message : String::from(""),
            awards : Vec::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnMessageChange(v) => {
                self.message = v;

            }
            Msg::OnOptionChange(v, option) => {
                match &option[..] {
                    "from" => self.options.from = v,
                    "to" => self.options.to = v,
                    "award" => self.options.award = v,
                    _ => println!()
                }
            }
            Msg::AddAward => {
                let award = OfficialAward{
                    from : self.options.from.clone(),
                    to : self.options.to.clone(),
                    award : self.options.award.clone(),
                    message : self.message.clone()
                };
                self.awards.push(award);
                self.message = String::from("")
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let populate_members = |(i, member): (usize, &String)| {
            html! {
                                           <option value={member}>
                                               {member}
                                               {"  "}<i style="color:red" class="fas fa-minus-circle"></i>
                                           </option>
                                   }
        };

        let view_awards = |(i, award): (usize, &OfficialAward)| {
            html!{
                <div>
                       <p style="font-weight:bold;">{award.award.clone()}{" TO: "}<span style="color:purple;font-size:20px">{award.to.clone()}</span></p>
                       <p style="margin-left:20px;word-break:break-all;font-style:italic;">{"\""}{award.message.clone()}{"\""}{" - "}<strong>{award.from.clone()}</strong></p>
                </div>
            }
        };

        let message_on_change = self.link.callback(|e: InputData| Msg::OnMessageChange(e.value));
        let give_award = self.link.callback(|e| Msg::AddAward);


        html! {
            <div class="mainContainer">
                <h1>{self.props.team_name.clone()}{" Team"}</h1>
                <div class="awards-container">
                    <div class="awards-inputs">
                        <div>
                            {"To  "}
                            <select value={self.options.to.clone()} onchange=self.link.callback(|e : ChangeData| {
                                let data = match e {
                                    ChangeData::Select(se) => se.value(),
                                    _ => unreachable!()
                                };
                                Msg::OnOptionChange(data, String::from("to"))
                            })
                            >
                                {for self.members.iter().enumerate().map(populate_members)}
                            </select>
                        </div>

                        <div>
                             {"From  "}
                              <select value={self.options.from.clone()} onchange=self.link.callback(|e : ChangeData| {
                                let data = match e {
                                    ChangeData::Select(se) => se.value(),
                                    _ => unreachable!()
                                };
                                Msg::OnOptionChange(data, String::from("from"))
                            })>
                                {for self.members.iter().enumerate().map(populate_members)}
                            </select>
                        </div>

                        <div>
                              {"Awarding  "}
                                 <select value={self.options.award.clone()} onchange=self.link.callback(|e : ChangeData| {
                                        let data = match e {
                                            ChangeData::Select(se) => se.value(),
                                            _ => unreachable!()
                                        };
                                        Msg::OnOptionChange(data, String::from("award"))
                                 })>
                                    <option value="🌼">{"🌼"}</option>
                                    <option value="⭐">{"⭐"}</option>
                                    <option value="👑">{"👑"}</option>
                                    <option value="🌹">{"🌹"}</option>
                                    <option value="🥇">{"🥇"}</option>
                                    <option value="🚀">{"🚀"}</option>
                                </select>
                        </div>
                        <button class="award-btn" onclick=give_award>{"Give AWW.ARD!"}</button>

                    </div>
                    <div class="awards-message">
                        <p>{"Message : "}</p>
                        <textarea value={self.message.clone()} oninput=message_on_change></textarea>
                    </div>
                    <div class="given-awards">
                        {
                            if self.awards.len() == 0 {html! {<p>{"Team awards will appear here"}</p>}}
                            else {html!{}}
                        }
                        {    for self.awards.iter().enumerate().map(view_awards)}
                    </div>
                </div>

            </div>
            }
        }
        }


