use clap::{Args, Parser};

// https://docs.rs/clap/latest/clap/

// // supported session types (if needed)
// #[derive(Args, Clone, Debug)]
// enum SessionType {
//     Tmux,
//     Tmuxinator,
//     Tmuxifier,
//     Tmuxp,
// }

/// CLI parser
#[derive(Parser, Debug)]
#[command(name = "rdrop")]
#[command(bin_name = "rdrop")]
pub struct Cli {
  /// Height for a newly created terminal (default: 45%)
  #[arg(short = 'h', long, default_value_t = 45)]
  pub height:                usize,
  /// Width for a newly created terminal (default: 100%)
  #[arg(short = 'w', long, default_value_t = 100)]
  pub width:                 usize,
  /// X offset for a newly created terminal (default: 0)
  #[arg(short = 'x', long, default_value_t = 0)]
  pub x_pos:                 isize,
  /// Y offset for a newly created terminal (default: 1)
  #[arg(short = 'y', long, default_value_t = 1)]
  pub y_pos:                 isize,
  /// Name for session (tmux/tmuxinator/tmuxifier/tmuxp)
  #[arg(short = 's', long)]
  pub name:                  Option<String>,
  /// Number or extra text for multiple dropdowns of same program
  #[arg(short = 'n', long = "num")]
  pub num_or_text:           Option<String>,
  /// Command to execute before creation
  #[arg(short = 'c', long = "pre-create")]
  pub pre_create:            Option<String>,
  /// Command to execute after creation
  #[arg(short = 'C', long = "post-create")]
  pub post_create:           Option<String>,
  /// Command to float the window before it is mapped
  #[arg(short = 'l', long = "pre-float")]
  pub pre_float:             Option<String>,
  /// Command to float the window after it is mapped
  #[arg(short = 'L', long = "post-float")]
  pub post_float:            Option<String>,
  /// Command to execute before mapping
  #[arg(short = 'p', long)]
  pub pre_map:               Option<String>,
  /// Command to execute after mapping
  #[arg(short = 'P', long)]
  pub post_map:              Option<String>,
  /// Command to execute before un-mapping
  #[arg(short = 'u', long)]
  pub pre_unmap:             Option<String>,
  /// Command to execute after un-mapping
  #[arg(short = 'U', long)]
  pub post_unmap:            Option<String>,
  /// Decoration/border size (format: XxY)
  #[arg(short = 'd', long)]
  pub decoration_size:       Option<String>,
  /// Fix saved geometry with auto_hide
  #[arg(short = 'S')]
  pub fix_geometry:          bool,
  /// Command to detect if the current window is floating (only with auto_hide)
  #[arg(short = 'i', long)]
  pub is_floating_cmd:       Option<String>,
  /// Flags/options for creating the terminal or window
  #[arg(short = 'f', long)]
  pub flags:                 Option<String>,
  /// Automatically detect window manager and set relevant options
  #[arg(short = 'a')]
  pub auto_detect_wm:        bool,
  /// Use with multiple monitors for dropdowns; converts percentages to monitor-relative values
  #[arg(short = 'm')]
  pub multiple_monitors:     bool,
  /// Use mouse pointer location for monitor detection
  #[arg(short = 't')]
  pub use_mouse_location:    bool,
  /// Always show/activate the window if it is not focused
  #[arg(short = 'A')]
  pub always_activate:       bool,
  /// Save geometry when hiding, restore when showing
  #[arg(short = 'r')]
  pub save_restore_geometry: bool,
  /// Reset to default geometry (do not use with -x, -y, -w, -h)
  #[arg(short = 'N')]
  pub reset_geometry:        bool,
  /// Set the window manager name to mimic another window manager (for use with -a)
  #[arg(long = "wm")]
  pub wm_name:               Option<String>,
  /// Manually specify the class of the window
  #[arg(long = "class")]
  pub window_class:          Option<String>,
  /// Set a new name for the dropdown window
  #[arg(long = "name")]
  pub window_name:           Option<String>,
  /// Clear saved window id (useful after accidentally making a window a dropdown)
  #[arg(long = "clear")]
  pub clear_window_id:       bool,
  /// Don't cancel auto-showing
  #[arg(long = "no-cancel")]
  pub no_cancel_auto_show:   bool,
  /// Set the timeout for window appearance (default: 10 seconds)
  #[arg(long = "timeout", default_value_t = 10)]
  pub timeout:               u64,
  /// Print debugging information
  #[arg(long = "debug")]
  pub debug:                 bool,
}
