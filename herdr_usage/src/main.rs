use anyhow::Result;
use clap::Parser;
use herdr_agent_quota::cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Command::AgyStatusline {
        state_dir: Some(path),
    } = &cli.command
    {
        std::env::set_var("HERDR_PLUGIN_STATE_DIR", path);
    }
    herdr_agent_quota::identity::adopt_alias_plugin_dirs();
    match cli.command {
        Command::Refresh {
            provider,
            force,
            json,
        } => herdr_agent_quota::refresh::run(&provider.providers(), force, json),
        Command::Watch {
            provider,
            interval_seconds,
            defer,
        } => herdr_agent_quota::refresh::watch(&provider.providers(), interval_seconds, defer),
        Command::Startup { provider } => herdr_agent_quota::refresh::startup(&provider.providers()),
        Command::Event => herdr_agent_quota::refresh::event(),
        Command::Focus => herdr_agent_quota::refresh::focus(),
        Command::Dashboard => herdr_agent_quota::dashboard::run(),
        Command::Settings => herdr_agent_quota::settings::run(),
        Command::Configure {
            check,
            apply,
            uninstall,
            reload,
            agent,
            watch_interval_seconds,
            sidebar_layout,
            quota_percent,
            sidebar_pacing,
            row_gap,
            fields,
            brand_colors,
            agent_order,
            low_quota_alert,
        } => {
            if reload && !(apply || uninstall) {
                anyhow::bail!("--reload requires --apply or --uninstall");
            }
            herdr_agent_quota::configure::run(
                check,
                apply,
                uninstall,
                &herdr_agent_quota::cli::AgentSelection::from_args_or_env(&agent),
                herdr_agent_quota::cli::ConfigureOptions {
                    watch_interval_seconds,
                    sidebar_layout,
                    quota_percent,
                    sidebar_pacing,
                    row_gap,
                    fields,
                    brand_colors,
                    agent_order,
                    low_quota_alert,
                },
            )?;
            if reload {
                herdr_agent_quota::herdr::reload_config()?;
                if apply {
                    herdr_agent_quota::refresh::startup(&herdr_agent_quota::model::Provider::ALL)?;
                }
            }
            Ok(())
        }
        Command::AgyStatusline { .. } => herdr_agent_quota::configure::agy::run_statusline_hook(),
        Command::OpenSettings => herdr_agent_quota::herdr::open_settings(),
    }
}
