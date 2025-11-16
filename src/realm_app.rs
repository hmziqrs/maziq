use std::time::Duration;

use tuirealm::{Application, EventListenerCfg};

use crate::{
    app::App as BusinessApp,
    catalog,
    components::{E2ETestComponent, MenuComponent, SoftwareComponent},
    messages::{AppMsg, ComponentId},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Menu,
    Software,
    E2ETest,
}

pub struct RealmApp {
    app: Application<ComponentId, AppMsg, tuirealm::NoUserEvent>,
    business: BusinessApp,
    current_screen: Screen,
    quit: bool,
    menu_component: MenuComponent,
    software_component: SoftwareComponent,
    e2e_component: E2ETestComponent,
}

impl RealmApp {
    pub fn new() -> Self {
        let app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_millis(200)),
        );

        let menu_component = MenuComponent::new();
        let software_component = SoftwareComponent::new(catalog::flattened_handles());
        let e2e_component = E2ETestComponent::new();

        let business = BusinessApp::new();

        Self {
            app,
            business,
            current_screen: Screen::Menu,
            quit: false,
            menu_component,
            software_component,
            e2e_component,
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit || self.business.quit
    }

    pub fn view(&mut self, terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>) {
        let _ = terminal.draw(|f| {
            let area = f.size();
            match self.current_screen {
                Screen::Menu => {
                    use tuirealm::MockComponent;
                    self.menu_component.view(f, area);
                }
                Screen::Software => {
                    use tuirealm::MockComponent;
                    self.software_component.view(f, area);
                }
                Screen::E2ETest => {
                    use tuirealm::MockComponent;
                    self.e2e_component.view(f, area);
                }
            }
        });
    }

    pub fn tick(&mut self) {
        // Poll business logic events
        self.business.poll_task_events();

        // Update component states from business logic
        self.sync_component_states();
    }

    pub fn handle_event(&mut self, ev: tuirealm::Event<tuirealm::NoUserEvent>) -> Option<AppMsg> {
        // Forward event to active component
        use tuirealm::Component;
        match self.current_screen {
            Screen::Menu => self.menu_component.on(ev),
            Screen::Software => self.software_component.on(ev),
            Screen::E2ETest => self.e2e_component.on(ev),
        }
    }

    pub fn update(&mut self, msg: AppMsg) {
        match msg {
            AppMsg::Quit => {
                self.quit = true;
            }
            AppMsg::GoToMenu => {
                self.current_screen = Screen::Menu;
                self.business.enter_menu();
            }
            AppMsg::GoToSoftwareCatalog => {
                self.current_screen = Screen::Software;
            }
            AppMsg::GoToE2ETest => {
                self.current_screen = Screen::E2ETest;
                self.e2e_component.reset();
            }
            AppMsg::ActivateMenuItem(index) => {
                self.business.activate_menu_index(index);
                // Check which screen to switch to
                match index {
                    0 => {
                        // Onboard Fresh - handled by business logic
                    }
                    1 => {
                        // Update template - handled by business logic
                    }
                    2 => {
                        // Config - handled by business logic
                    }
                    3 => {
                        // Software catalog
                        self.current_screen = Screen::Software;
                    }
                    4 => {
                        // Versions - handled by business logic
                    }
                    5 => {
                        // E2E Test
                        self.current_screen = Screen::E2ETest;
                        self.e2e_component.reset();
                        self.business.message = "Select a software: 1=Neovim, 2=btop".into();
                    }
                    _ => {}
                }
            }
            AppMsg::InstallSoftware(id) => {
                self.business.run_action(
                    crate::manager::ActionKind::Install,
                    vec![id],
                );
            }
            AppMsg::UpdateSoftware(id) => {
                self.business.run_action(
                    crate::manager::ActionKind::Update,
                    vec![id],
                );
            }
            AppMsg::UninstallSoftware(id) => {
                self.business.run_action(
                    crate::manager::ActionKind::Uninstall,
                    vec![id],
                );
            }
            AppMsg::InstallAllMissing => {
                self.business.install_all_missing();
            }
            AppMsg::RefreshStatuses => {
                self.business.refresh_statuses_with_feedback();
            }
            AppMsg::ToggleTaskView => {
                self.business.toggle_task_view();
            }
            AppMsg::SelectE2ESoftware(software) => {
                self.business.select_e2e_software(software);
                self.e2e_component.set_software(software);
            }
            AppMsg::ToggleE2EInstall => {
                self.business.toggle_e2e_step();
                self.e2e_component.toggle_current_step();
            }
            AppMsg::ToggleE2EUpdate => {
                self.business.toggle_e2e_step();
                self.e2e_component.toggle_current_step();
            }
            AppMsg::ToggleE2ERemove => {
                self.business.toggle_e2e_step();
                self.e2e_component.toggle_current_step();
            }
            AppMsg::ExecuteE2E => {
                self.business.execute_e2e();
                self.e2e_component.set_executing(true);
            }
            AppMsg::None => {}
        }

        // Sync states after update
        self.sync_component_states();
    }

    fn sync_component_states(&mut self) {
        // Update Menu component
        self.menu_component.set_message(self.business.message().to_string());
        self.menu_component.set_log_lines(self.business.log().to_vec());
        self.menu_component.set_show_tasks(self.business.show_tasks());
        self.menu_component.set_task_lines(self.format_task_logs());

        // Update Software component
        let statuses = self
            .business
            .handles()
            .iter()
            .map(|h| (h.id, self.business.status_for(h.id)))
            .collect();
        self.software_component.set_statuses(statuses);
        self.software_component.set_message(self.business.message().to_string());
        self.software_component.set_log_lines(self.business.log().to_vec());
        self.software_component.set_show_tasks(self.business.show_tasks());
        self.software_component.set_task_lines(self.format_task_logs());

        // Update E2E component
        self.e2e_component.set_task_lines(self.format_task_logs());
        self.e2e_component.set_executing(self.business.e2e_executing());
    }

    fn format_task_logs(&self) -> Vec<String> {
        let mut lines = Vec::new();
        for task in self.business.task_logs() {
            lines.push(format!("#{} {}", task.id, task.label));
            for line in &task.lines {
                lines.push(format!("  {}", line));
            }
        }
        lines
    }
}
