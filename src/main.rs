use iced::time::{self, Duration};
use iced::widget::{
    Column, button, column, container, progress_bar, row, text, text_input, toggler,
};
use iced::{Border, Center, Element, Fill, Left, Right, Bottom, Subscription, Task, Theme};
use native_dialog::{MessageDialog, MessageType};

mod gpu;
use gpu::Gpu;

const FONT_SIZE_SM: f32 = 15.0;
const FONT_SIZE_MED: f32 = 20.0;
const FONT_SIZE_LG: f32 = 25.0;

const DARK_THEME: Theme = Theme::Oxocarbon;

struct Tweaks {
    theme: Theme,

    power_watts_input: String,
    core_offset_input: String,
    mem_offset_input: String,

    core_offset_real: String,
    mem_offset_real: String,

    power_watts: String,
    gpu_temp: String,

    mem_usage: String,

    toggler_value: bool,
    nvml: Gpu,
}

#[derive(Debug, Clone)]
enum Message {
    PowerChanged(String),
    CoreChanged(String),
    MemChanged(String),
    TogglerToggled(bool),
    ApplyPressed,
    UpdateGPUStats,
}

impl Tweaks {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                theme: DARK_THEME,
                power_watts_input: 0.to_string(),
                core_offset_input: 0.to_string(),
                mem_offset_input: 0.to_string(),
                core_offset_real: 0.to_string(),
                mem_offset_real: 0.to_string(),
                power_watts: 0.to_string(),
                gpu_temp: 0.to_string(),
                mem_usage: 0.to_string(),
                toggler_value: true,
                nvml: Gpu::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::PowerChanged(value) => {
                self.power_watts_input = value;
            }
            Message::CoreChanged(value) => {
                self.core_offset_input = value;
            }
            Message::MemChanged(value) => {
                self.mem_offset_input = value;
            }
            Message::TogglerToggled(value) => {
                self.toggler_value = value;
                if self.toggler_value {
                    self.theme = DARK_THEME;
                } else {
                    self.theme = Theme::Light;
                }
            }
            Message::ApplyPressed => {
                // run the overclock application function
                let result = self.nvml.apply_oc(
                    self.core_offset_input.clone(),
                    self.mem_offset_input.clone(),
                );

                // check the result we get back to handle errors
                match result {
                    Ok(_m) => {}
                    Err(error) => {
                        let _ = MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Error")
                            .set_text(&format!("Error while setting the overclock. Make sure the app is running as sudo. Error Code: {error:?}"))
                            .show_alert();
                    }
                }
            }

            Message::UpdateGPUStats => {
                // run the actual upate in the background
                self.nvml.update_gpu_info();

                self.power_watts = self.nvml.power_watts.clone();
                self.power_watts.push_str(" W");
                self.gpu_temp = self.nvml.gpu_temp.clone();

                self.mem_usage = String::from("");
                self.mem_usage.push_str(&self.nvml.gpu_mem_used);
                self.mem_usage.push_str(" MiB/");
                self.mem_usage.push_str(&self.nvml.gpu_mem_total);
                self.mem_usage.push_str(" MiB");

                let core_off = self.nvml.get_gpu_offset();
                match core_off {
                    Ok(core_off) => self.core_offset_real = core_off.to_string(),
                    Err(e) => {
                        let _ = MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Error")
                            .set_text(&format!(
                                "Error reading the core clock offset. Error Code {e:?}"
                            ))
                            .show_alert();
                    }
                }

                let mem_off = self.nvml.get_mem_offset();
                match mem_off {
                    Ok(mem_off) => self.mem_offset_real = mem_off.to_string(),
                    Err(e) => {
                        let _ = MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Error")
                            .set_text(&format!(
                                "Error reading the mem clock offset. Error Code {e:?}"
                            ))
                            .show_alert();
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let power_input = text_input("0", &self.power_watts_input)
            .on_input(Message::PowerChanged)
            .padding(10)
            .size(FONT_SIZE_MED);

        let core_input = text_input("0", &self.core_offset_input)
            .on_input(Message::CoreChanged)
            .padding(10)
            .size(FONT_SIZE_MED);

        let mem_input = text_input("0", &self.mem_offset_input)
            .on_input(Message::MemChanged)
            .padding(10)
            .size(FONT_SIZE_MED);

        let toggler = toggler(self.toggler_value)
            .label("Dark Mode")
            .on_toggle(Message::TogglerToggled)
            .spacing(FONT_SIZE_MED);

        let styled_button = |label| {
            button(text(label).width(50).center())
                .padding(15)
                .on_press(Message::ApplyPressed)
        };

        let apply_button = styled_button("Apply");

        //------------------------ Info Section -------------------------------
        let info_labels = column![
            row![
                text("Name").size(FONT_SIZE_MED).width(100),
                container(text(self.nvml.get_gpu_name()).size(FONT_SIZE_MED))
                    .style(container::rounded_box)
                    .padding(5)
                    .align_x(Center)
                    .width(Fill)
            ]
            .spacing(12)
            .align_y(Center),
            row![
                text("Driver").size(FONT_SIZE_MED).width(100),
                container(text(self.nvml.get_driver_version()).size(FONT_SIZE_MED))
                    .style(container::rounded_box)
                    .padding(5)
                    .align_x(Center)
                    .width(Fill)
            ]
            .spacing(12)
            .align_y(Center),
            row![
                text("Memory").size(FONT_SIZE_MED).width(100),
                container(text(&self.mem_usage).size(FONT_SIZE_MED))
                    .style(container::rounded_box)
                    .padding(5)
                    .align_x(Center)
                    .width(Fill)
            ]
            .spacing(12)
            .align_y(Center),
            row![
                text("GPU Use").size(FONT_SIZE_MED).width(100),
                progress_bar(0.0..=100.0, self.nvml.gpu_utilization.clone() as f32).width(Fill),
                container(row![
                    text(self.nvml.gpu_utilization.to_string()).size(FONT_SIZE_MED),
                    text("%").size(FONT_SIZE_MED)
                ])
                .style(container::rounded_box)
                .padding(5)
                .align_x(Center)
            ]
            .spacing(12)
            .align_y(Center),
            row![
                text("Mem Use").size(FONT_SIZE_MED).width(100),
                progress_bar(0.0..=100.0, self.nvml.mem_utilization.clone() as f32).width(Fill),
                container(row![
                    text(self.nvml.mem_utilization.to_string()).size(FONT_SIZE_MED),
                    text("%").size(FONT_SIZE_MED)
                ])
                .style(container::rounded_box)
                .padding(5)
                .align_x(Center)
            ]
            .spacing(12)
            .align_y(Center),
        ]
        .spacing(12)
        .align_x(Left)
        .padding(10);

        let info_container = column![
            text("Info").size(FONT_SIZE_LG).align_x(Center),
            container(info_labels).style(custom_container)
        ]
        .align_x(Left)
        .padding(10);
        //---------------------------------------------------------------------
        //-------------------------- Clocks Section ---------------------------

        // set up some labels for the gpu clocks section
        let graphics_labels = ["Graphics", "SM", "Memory", "Video"];

        // make a column to hold all the clock speeds
        let mut clock_data = Column::new().spacing(12).align_x(Left).padding(10);

        // loop through the labels to retreive the clock speeds and create rows
        for (index, label) in graphics_labels.iter().enumerate() {
            clock_data = clock_data.push(
                row![
                    text(*label).size(FONT_SIZE_MED).width(100),
                    container(text(&self.nvml.clock_speed_array[index]).size(FONT_SIZE_MED))
                        .style(container::rounded_box)
                        .padding(5)
                        .width(Fill)
                        .align_x(Center),
                    container(text(&self.nvml.clock_speed_max_array[index]).size(FONT_SIZE_MED))
                        .style(container::rounded_box)
                        .padding(5)
                        .width(Fill)
                        .align_x(Center),
                ]
                .spacing(12)
                .align_y(Center),
            );
        }

        let clocks_container = column![
            text("Clocks (Current/Max)")
                .size(FONT_SIZE_LG)
                .align_x(Center),
            container(clock_data).style(custom_container)
        ]
        .align_x(Left)
        .padding(10);
        // --------------------------------------------------------------------
        // ----------------------- Overclocking Section -----------------------

        let oc_data = column![
            row![
                column![
                    text("Set Core Offset (MHz)").size(FONT_SIZE_SM),
                    text_input("0", &self.core_offset_input)
                        .on_input(Message::CoreChanged)
                        .padding(10)
                        .size(FONT_SIZE_MED)
                ],
                column![
                    text("Set Mem Offset (MHz)").size(FONT_SIZE_SM),
                    text_input("0", &self.mem_offset_input)
                        .on_input(Message::MemChanged)
                        .padding(10)
                        .size(FONT_SIZE_MED)
                ],
                button(text("Apply").width(50).center())
                    .padding(15)
                    .on_press(Message::ApplyPressed)
            ]
            .spacing(12)
            .align_y(Bottom)
            .padding(10),
            row![
                column![
                    text("Core Offset (MHz)").size(FONT_SIZE_SM),
                    container(text(&self.core_offset_real).size(FONT_SIZE_MED))
                        .style(container::rounded_box)
                        .padding(5)
                        .width(Fill)
                        .align_x(Center),
                ],
                column![
                    text("Mem Offset (MHz)").size(FONT_SIZE_SM),
                    container(text(&self.mem_offset_real).size(FONT_SIZE_MED))
                        .style(container::rounded_box)
                        .padding(5)
                        .width(Fill)
                        .align_x(Center),
                ],
                //button(text("Apply").width(50).center())
                //    .padding(15)
                //    .on_press(Message::ApplyPressed)
            ]
            .spacing(12)
            .align_y(Center)
            .padding(10),
        ];

        let oc_container = column![
            text("Overclock").size(FONT_SIZE_LG).align_x(Center),
            container(oc_data).style(custom_container)
        ]
        .align_x(Left)
        .padding(10);
        //---------------------------------------------------------------------

        let bottom_row = row![toggler, apply_button].spacing(10);

        let settings_column = column![
            text("Settings").size(FONT_SIZE_LG),
            row![text("Core Offset ").size(FONT_SIZE_MED), core_input],
            row![text("Mem Offset ").size(FONT_SIZE_MED), mem_input],
            row![text("Power ").size(FONT_SIZE_MED), power_input],
            bottom_row
        ]
        .spacing(10)
        .align_x(Right)
        .padding(10);

        let left_column = column![info_container, clocks_container, oc_container];

        let settings_column_container = container(settings_column).style(custom_container);

        let content = row![left_column, settings_column_container].spacing(15);

        content.into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn gpu_update_stats(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(300)).map(|_x| Message::UpdateGPUStats)
    }
}

fn main() -> iced::Result {
    iced::application("Nvidia Tweaker", Tweaks::update, Tweaks::view)
        .subscription(Tweaks::gpu_update_stats)
        .theme(Tweaks::theme)
        .run_with(Tweaks::new)
}

// implement a custom container theme
fn custom_container(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.base.color.into()),
        border: Border {
            width: 3.0,
            radius: 2.0.into(),
            color: palette.background.strong.color,
        },
        ..container::Style::default()
    }
}
