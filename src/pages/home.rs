use yew::prelude::*;
use yew::services::ConsoleService;
use crate::app_router::{AppRoute,Link};
use yew::format::Json;
use yew::services::storage::Area;
use yew::services::StorageService;

use crate::data_structures;


pub struct Home {
    link: ComponentLink<Self>,
    inputs : data_structures::MainPageInputs,
    storage: StorageService,
    database: data_structures::Database,
}

pub enum Msg {
    MemberInputOnChange(String),
    TeamInputOnChange(String),
    AddTeamMember,
    RemoveMember(usize)
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let Json(database) = storage.restore(data_structures::KEY);
        let database = database.unwrap_or_else(|_| data_structures::Database::new());

        Self {
            link,
            inputs : data_structures::MainPageInputs {
                team_name_input : String::from("Watermelon"),
                team_member_input : String::from("Anonymous")
            },
            storage,
            database
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MemberInputOnChange(v) => {
                ConsoleService::log("Input has changed.");
                self.inputs.team_member_input = v;
            }
            Msg::TeamInputOnChange(v) => {
                ConsoleService::log("Team Input has changed.");
                self.inputs.team_name_input = v;
            }
            Msg::AddTeamMember => {
                if self.inputs.team_member_input != "" {
                    self.database.members.push(self.inputs.team_member_input.clone());
                    self.storage.store(data_structures::KEY, Json(&self.database));

                    self.inputs.team_member_input = String::from("");
                }
            }
            Msg::RemoveMember(member_id) => {
                self.database.members.remove(member_id);
                self.storage.store(data_structures::KEY, Json(&self.database));
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let member_on_change = self.link.callback(|e:InputData| Msg::MemberInputOnChange(e.value));
        let team_on_change = self.link.callback(|e:InputData| Msg::TeamInputOnChange(e.value));


        let view_members = |(i, member): (usize, &String)| {
            html!{
                                      <div>
                                           <li onclick=self.link.callback(move |_| Msg::RemoveMember(i))>
                                               {member}
                                                {"  "}<i style="color:red" class="fas fa-minus-circle"></i>
                                           </li>

                                       </div>
                                   }
        };

        let display_proceed = move || -> Html {
            if self.database.members.len() >=2 {
                html! {
                            <div class="proceed" style="align-self:flex-end; margin-top:20px; font-size:18px;">
                                <Link route=AppRoute::Awards(self.inputs.team_name_input.clone())><span>{"Proceed to AWW.ARDS "}<i class="fas fa-arrow-circle-right"></i></span></Link>
                            </div>
                        }
            }
            else {
                html! { <p>{"Please add at least 2 members before proceeding."}</p>}
            }
        };

        html! {
            <div class="mainContainer">
                <h1><span style="color:#e9c46a;">{"Aww"}</span><span style="color:#e76f51">{".wards"}</span></h1>
                <p class="subText">{" Your bi-monthly teammate appreciation platform"}</p>
                <div style="display:flex;width:100%;height:220px;">
                    <div class="inputs">
                        <div>
                            <p>{"Begin by specifying your team name and adding members to your team."}</p>
                        </div>
                        <div>
                             <label for="team-name">{"Team Name"}</label><br/>
                             <input oninput=team_on_change type="text" name="team-name" id="team-name-id" value=self.inputs.team_name_input/>
                        </div>
                        <div>
                             <label for="team-member">{"Team Member"}</label><br/>
                             <input oninput=member_on_change type="text" name="team-member" id="team-member-id" value=self.inputs.team_member_input/>
                             <span onclick=self.link.callback(|_| Msg::AddTeamMember) style="margin-left:10px;"><i class="fas fa-plus"></i></span>
                        </div>
                    </div>
                    <div class="members">
                        <p>{"Your team members will appear here"}</p>
                        {for self.database.members.iter().enumerate().map(view_members)}
                    </div>
                </div>
                {display_proceed()}
            </div>
        }
    }
}