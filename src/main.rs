use iced::time::{self, Duration};
use iced::widget::{
    button, center, column, container, progress_bar, row, text, text_input, toggler,
};
use iced::{Alignment, Border, Center, Element, Fill, Left, Right, Subscription, Task, Theme, Top};

mod gpu;
use gpu::Gpu;

const FONT_SIZE_SM: f32 = 15.0;
const FONT_SIZE_MED: f32 = 20.0;
const FONT_SIZE_LG: f32 = 25.0;

struct Tweaks {
    theme: Theme,
    power_watts_input: String,
    core_offset_input: String,
    mem_offset_input: String,

    power_watts: String,
    core_freq: String,
    mem_freq: String,
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
                theme: Theme::TokyoNight,
                power_watts_input: 0.to_string(),
                core_offset_input: 0.to_string(),
                mem_offset_input: 0.to_string(),
                power_watts: 0.to_string(),
                core_freq: 0.to_string(),
                mem_freq: 0.to_string(),
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
                    self.theme = Theme::TokyoNight;
                } else {
                    self.theme = Theme::Light;
                }
            }
            Message::ApplyPressed => {}
            Message::UpdateGPUStats => {
                // run the actual upate in the background
                self.nvml.update_gpu_info();

                self.power_watts = self.nvml.power_watts.clone();
                self.power_watts.push_str(" W");
                self.core_freq = self.nvml.core_freq.clone();
                self.mem_freq = self.nvml.mem_freq.clone();
                self.gpu_temp = self.nvml.gpu_temp.clone();

                self.mem_usage = String::from("");
                self.mem_usage.push_str(&self.nvml.gpu_mem_used);
                self.mem_usage.push_str(" MiB/");
                self.mem_usage.push_str(&self.nvml.gpu_mem_total);
                self.mem_usage.push_str(" MiB");
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
                text("GPU %").size(FONT_SIZE_MED).width(100),
                progress_bar(0.0..=100.0, self.nvml.gpu_utilization.clone() as f32).width(Fill),
                container(text(self.nvml.gpu_utilization.to_string()).size(FONT_SIZE_MED))
                    .style(container::rounded_box)
                    .padding(5)
                    .align_x(Center)
            ]
            .spacing(12)
            .align_y(Center),
            row![
                text("Mem %").size(FONT_SIZE_MED).width(100),
                progress_bar(0.0..=100.0, self.nvml.mem_utilization.clone() as f32).width(Fill),
                container(text(self.nvml.mem_utilization.to_string()).size(FONT_SIZE_MED))
                    .style(container::rounded_box)
                    .padding(5)
                    .align_x(Center)
            ]
            .spacing(12)
            .align_y(Center),
            //text("Encoder %").size(FONT_SIZE_MED).width(100),
            //text("Decoder %").size(FONT_SIZE_MED).width(100),
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

        let clocks_data = column![row![
            text("Graphics").size(FONT_SIZE_MED).width(100),
            container(text(&self.core_freq).size(FONT_SIZE_MED))
                .style(container::rounded_box)
                .padding(5)
                .width(Fill)
                .align_x(Center),
            container(text(&self.nvml.max_core_freq).size(FONT_SIZE_MED))
                .style(container::rounded_box)
                .padding(5)
                .width(Fill)
                .align_x(Center),

        ].spacing(12).padding(10).align_y(Center)]
        .spacing(12)
        .align_x(Left)
        .padding(10);

        let clocks_container = column![
            text("Clocks (Current/Max)")
                .size(FONT_SIZE_LG)
                .align_x(Center),
            container(clocks_data).style(custom_container)
        ]
        .align_x(Left)
        .padding(10);

        let bottom_row = row![toggler, apply_button].spacing(10);

        //let stats_column = column![
        //    text("GPU Stats").size(FONT_SIZE_LG),
        //    row![
        //        text("Core Clock ").size(FONT_SIZE_LG),
        //        container(text(self.core_freq.clone()).size(FONT_SIZE_MED))
        //            .style(container::rounded_box)
        //            .center(60) //.width(60)
        //    ],
        //    row![
        //        text("Memory Clock ").size(FONT_SIZE_MED),
        //        text(self.mem_freq.clone())
        //    ],
        //    row![
        //        text("Power ").size(FONT_SIZE_MED),
        //        text(self.power_watts.clone())
        //    ],
        //    row![
        //        text("Temp ").size(FONT_SIZE_MED),
        //        text(self.gpu_temp.clone())
        //    ],
        //]
        //.spacing(10);
        //
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

        let left_column = column![info_container, clocks_container];

        let settings_column_container = container(settings_column).style(custom_container);

        let content = row![left_column, settings_column_container].spacing(15);

        //center(content).into()
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
