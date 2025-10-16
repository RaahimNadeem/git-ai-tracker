use crate::authorship::stats::stats_command;
use crate::authorship::working_log::AgentId;
use crate::commands;
use crate::commands::checkpoint_agent::agent_preset::{
    AgentCheckpointFlags, AgentCheckpointPreset, AgentRunResult, ClaudePreset, CursorPreset,
    GithubCopilotPreset,
};
use crate::config;
use crate::git::find_repository;
use crate::git::find_repository_in_path;
use crate::git::repository::exec_git;
use crate::utils::Timer;
use std::io::IsTerminal;

pub fn handle_git_ai(args: &[String]) {
    if args.is_empty() {
        print_help();
        return;
    }
    let timer = Timer::default();

    match args[0].as_str() {
        "help" | "--help" | "-h" => {
            print_help();
        }
        "version" | "--version" | "-v" => {
            println!(env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        "stats-delta" => {
            handle_stats_delta(&args[1..]);
        }
        "stats" => {
            handle_stats(&args[1..]);
        }
        "stats-display" | "show-ai" => {
            handle_stats_display(&args[1..]);
        }
        "stats-repo" | "stats-aggregate" => {
            handle_stats_repo(&args[1..]);
        }
        "checkpoint" => {
            let end = timer.start("git-ai checkpoint");
            handle_checkpoint(&args[1..]);
            end();
        }
        "blame" => {
            handle_ai_blame(&args[1..]);
        }
        "git-path" => {
            let config = config::Config::get();
            println!("{}", config.git_cmd());
            std::process::exit(0);
        }
        "install-hooks" => {
            if let Err(e) = commands::install_hooks::run(&args[1..]) {
                eprintln!("Install hooks failed: {}", e);
                std::process::exit(1);
            }
        }

        "squash-authorship" => {
            commands::squash_authorship::handle_squash_authorship(&args[1..]);
        }
        _ => {
            println!("Unknown git-ai command: {}", args[0]);
            std::process::exit(1);
        }
    }
}

fn print_help() {
    eprintln!("git-ai-tracker - git proxy with AI authorship tracking");
    eprintln!("");
    eprintln!("Usage: git-ai-tracker <git or git-ai-tracker command> [args...]");
    eprintln!("");
    eprintln!("Commands:");
    eprintln!("  checkpoint         checkpoint working changes and specify author");
    eprintln!("    Presets: github-copilot, cursor. Debug/Testing presets mock_ai");
    eprintln!("    --show-working-log    Display current working log");
    eprintln!("    --reset               Reset working log");
    eprintln!("  blame              [override] git blame with AI authorship tracking");
    eprintln!(
        "  commit             [wrapper] pass through to 'git commit' with git-ai-tracker hooks"
    );
    eprintln!("  stats              Show AI authorship statistics for a commit");
    eprintln!("    <commit>               Optional commit SHA (defaults to current HEAD)");
    eprintln!("    --json                 Output in JSON format");
    eprintln!("  stats-display      Show prominent AI% display for a commit (alias: show-ai)");
    eprintln!("    <commit>               Optional commit SHA (defaults to current HEAD)");
    eprintln!("  stats-repo         Show aggregate AI% across entire repository history (alias: stats-aggregate)");
    eprintln!("    [--limit N]            Limit to last N commits (default: all commits)");
    eprintln!("    [--branch name]        Analyze specific branch (default: current branch)");
    eprintln!("    [--since date]         Only commits after date (e.g., '2024-01-01', '1 week ago')");
    eprintln!("    [--json]               Output in JSON format");
    eprintln!("  install-hooks      Install git hooks for AI authorship tracking");
    eprintln!("  squash-authorship  Generate authorship from squashed commits");
    eprintln!("    <branch> <new_sha> <old_sha>  Required: branch, new commit SHA, old commit SHA");
    eprintln!("    --dry-run             Show what would be done without making changes");
    eprintln!("");
    std::process::exit(0);
}

fn handle_checkpoint(args: &[String]) {
    let mut repository_working_dir = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();

    // Parse checkpoint-specific arguments
    let mut author = None;
    let mut show_working_log = false;
    let mut reset = false;
    let mut prompt_id = None;
    let mut hook_input = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--author" => {
                if i + 1 < args.len() {
                    author = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --author requires a value");
                    std::process::exit(1);
                }
            }
            "--show-working-log" => {
                show_working_log = true;
                i += 1;
            }
            "--reset" => {
                reset = true;
                i += 1;
            }
            "--prompt-id" => {
                if i + 1 < args.len() {
                    prompt_id = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --prompt-id requires a value");
                    std::process::exit(1);
                }
            }
            "--hook-input" => {
                if i + 1 < args.len() {
                    hook_input = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --hook-input requires a value");
                    std::process::exit(1);
                }
            }

            _ => {
                i += 1;
            }
        }
    }

    let mut agent_run_result = None;
    // Handle preset arguments after parsing all flags
    if !args.is_empty() {
        match args[0].as_str() {
            "claude" => {
                match ClaudePreset.run(AgentCheckpointFlags {
                    prompt_id: prompt_id.clone(),
                    hook_input: hook_input.clone(),
                }) {
                    Ok(agent_run) => {
                        agent_run_result = Some(agent_run);
                    }
                    Err(e) => {
                        eprintln!("Claude preset error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            "cursor" => {
                match CursorPreset.run(AgentCheckpointFlags {
                    prompt_id: prompt_id.clone(),
                    hook_input: hook_input.clone(),
                }) {
                    Ok(agent_run) => {
                        if agent_run.is_human {
                            agent_run_result = None;
                            if agent_run.repo_working_dir.is_some() {
                                repository_working_dir = agent_run.repo_working_dir.unwrap();
                            }
                        } else {
                            agent_run_result = Some(agent_run);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error running Cursor preset: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            "github-copilot" => {
                match GithubCopilotPreset.run(AgentCheckpointFlags {
                    prompt_id: prompt_id.clone(),
                    hook_input: hook_input.clone(),
                }) {
                    Ok(agent_run) => {
                        agent_run_result = Some(agent_run);
                    }
                    Err(e) => {
                        eprintln!("Github Copilot preset error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            "mock_ai" => {
                agent_run_result = Some(AgentRunResult {
                    agent_id: AgentId {
                        tool: "some-ai".to_string(),
                        id: "ai-thread".to_string(),
                        model: "unknown".to_string(),
                    },
                    is_human: false,
                    transcript: None,
                    repo_working_dir: None,
                });
            }
            _ => {}
        }
    }

    let final_working_dir = agent_run_result
        .as_ref()
        .and_then(|r| r.repo_working_dir.clone())
        .unwrap_or_else(|| repository_working_dir);
    // Find the git repository
    let repo = match find_repository_in_path(&final_working_dir) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to find repository: {}", e);
            std::process::exit(1);
        }
    };

    // Get the current user name from git config
    let default_user_name = match repo.config_get_str("user.name") {
        Ok(Some(name)) if !name.trim().is_empty() => name,
        _ => {
            eprintln!("Warning: git user.name not configured. Using 'unknown' as author.");
            "unknown".to_string()
        }
    };

    let final_author = author.as_ref().unwrap_or(&default_user_name);

    if let Err(e) = commands::checkpoint::run(
        &repo,
        final_author,
        show_working_log,
        reset,
        false,
        agent_run_result,
    ) {
        eprintln!("Checkpoint failed: {}", e);
        std::process::exit(1);
    }
}

fn handle_stats_delta(args: &[String]) {
    // Parse stats-delta-specific arguments
    let mut json_output = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                json_output = true;
                i += 1;
            }
            _ => {
                eprintln!("Unknown stats-delta argument: {}", args[i]);
                std::process::exit(1);
            }
        }
    }

    // TODO: Do we have any 'global' args for the stats-delta?
    // Find the git repository
    let repo = match find_repository(&Vec::<String>::new()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to find repository: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = commands::stats_delta::run(&repo, json_output) {
        eprintln!("Stats delta failed: {}", e);
        std::process::exit(1);
    }
}

fn handle_ai_blame(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: blame requires a file argument");
        std::process::exit(1);
    }

    // TODO: Do we have any 'global' args for the ai-blame?
    // Find the git repository
    let repo = match find_repository(&Vec::<String>::new()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to find repository: {}", e);
            std::process::exit(1);
        }
    };

    // Parse blame arguments
    let (file_path, options) = match commands::blame::parse_blame_args(args) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to parse blame arguments: {}", e);
            std::process::exit(1);
        }
    };

    // Check if this is an interactive terminal
    let is_interactive = std::io::stdout().is_terminal();

    if is_interactive && options.incremental {
        // For incremental mode in interactive terminal, we need special handling
        // This would typically involve a pager like less
        eprintln!("Error: incremental mode is not supported in interactive terminal");
        std::process::exit(1);
    }

    if let Err(e) = repo.blame(&file_path, &options) {
        eprintln!("Blame failed: {}", e);
        std::process::exit(1);
    }
}

fn handle_stats(args: &[String]) {
    // Parse stats-specific arguments
    let mut json_output = false;
    let mut commit_sha = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                json_output = true;
                i += 1;
            }
            _ => {
                // First non-flag argument is treated as commit SHA
                if commit_sha.is_none() {
                    commit_sha = Some(args[i].clone());
                    i += 1;
                } else {
                    eprintln!("Unknown stats argument: {}", args[i]);
                    std::process::exit(1);
                }
            }
        }
    }

    // Find the git repository
    let repo = match find_repository(&Vec::<String>::new()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Failed to find repository: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = stats_command(&repo, commit_sha.as_deref(), json_output) {
        match e {
            crate::error::GitAiError::Generic(msg) if msg.starts_with("No commit found:") => {
                eprintln!("{}", msg);
            }
            _ => {
                eprintln!("Stats failed: {}", e);
            }
        }
        std::process::exit(1);
    }
}

fn handle_stats_display(args: &[String]) {
    use crate::authorship::stats::stats_for_commit_stats;
    use crate::git::refs::show_authorship_note;
    
    // Find the repository starting from the current directory
    let repo = match find_repository(&Vec::<String>::new()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Error finding repository: {}", e);
            std::process::exit(1);
        }
    };

    // Parse commit argument
    let commit_sha = args.first().map(|s| s.as_str());
    
    // Determine target commit
    let (target, refname) = if let Some(sha) = commit_sha {
        match repo.revparse_single(sha) {
            Ok(commit_obj) => {
                let full_sha = commit_obj.id();
                (full_sha, format!("{}", sha))
            }
            Err(_) => {
                eprintln!("Error: No commit found: {}", sha);
                std::process::exit(1);
            }
        }
    } else {
        match repo.head() {
            Ok(head) => {
                match head.target() {
                    Ok(target) => {
                        let name = head.name().unwrap_or("HEAD").to_string();
                        (target, name)
                    }
                    Err(e) => {
                        eprintln!("Error: Failed to get HEAD target: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: Failed to get HEAD: {}", e);
                std::process::exit(1);
            }
        }
    };

    // Get stats
    let stats = match stats_for_commit_stats(&repo, &target, &refname) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error getting stats: {}", e);
            std::process::exit(1);
        }
    };

    // Calculate AI percentage
    let total_additions = stats.git_diff_added_lines;
    let ai_percentage = if total_additions > 0 {
        ((stats.ai_additions as f64 / total_additions as f64) * 100.0).round() as u32
    } else {
        0
    };

    // Display the AI% prominently!
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                                                          ║");
    println!("║           🤖 AI CONTRIBUTION TRACKER 🤖                  ║");
    println!("║                                                          ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║                                                          ║");
    
    // Big AI percentage display
    let ai_display = format!("           AI Contribution: {}%", ai_percentage);
    println!("║  {}  ║", format!("\x1b[1;36m{}\x1b[0m", ai_display));
    println!("║                                                          ║");
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║  Commit: {:<48}║", &target[..8.min(target.len())]);
    println!("║  Ref: {:<51}║", &refname[..51.min(refname.len())]);
    println!("╠══════════════════════════════════════════════════════════╣");
    println!("║                                                          ║");
    println!("║  📊 Breakdown:                                           ║");
    println!("║                                                          ║");
    println!("║    AI Lines:       {:>4} / {:<4} ({:>3}%)                  ║", 
        stats.ai_additions, total_additions, ai_percentage);
    println!("║    Human Lines:    {:>4} / {:<4} ({:>3}%)                  ║", 
        stats.human_additions, total_additions, 
        if total_additions > 0 { ((stats.human_additions as f64 / total_additions as f64) * 100.0).round() as u32 } else { 0 });
    println!("║    Mixed Lines:    {:>4}                                  ║", stats.mixed_additions);
    println!("║                                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("");

    // Show raw metadata from notes if available
    if let Some(notes) = show_authorship_note(&repo, &target) {
        if notes.contains("--- AI Contribution Metadata ---") {
            println!("📝 Git Notes Metadata:");
            println!("");
            for line in notes.lines() {
                if line.starts_with("AI-") || line.starts_with("Human-") || line.starts_with("Mixed-") || line.starts_with("Generated:") {
                    println!("   {}", line);
                }
            }
            println!("");
        }
    }

    println!("💡 Tip: Use 'git notes show {}' to see full authorship data", &target[..8.min(target.len())]);
    println!("");
}

fn handle_stats_repo(args: &[String]) {
    use crate::authorship::stats::stats_for_commit_stats;
    use serde_json::json;

    // Parse arguments
    let mut limit: Option<usize> = None;
    let mut branch: Option<String> = None;
    let mut since: Option<String> = None;
    let mut json_output = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--limit" => {
                if i + 1 < args.len() {
                    limit = args[i + 1].parse().ok();
                    i += 2;
                } else {
                    eprintln!("Error: --limit requires a number");
                    std::process::exit(1);
                }
            }
            "--branch" => {
                if i + 1 < args.len() {
                    branch = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --branch requires a branch name");
                    std::process::exit(1);
                }
            }
            "--since" => {
                if i + 1 < args.len() {
                    since = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --since requires a date");
                    std::process::exit(1);
                }
            }
            "--json" => {
                json_output = true;
                i += 1;
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                std::process::exit(1);
            }
        }
    }

    // Find the repository
    let repo = match find_repository(&Vec::<String>::new()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Error finding repository: {}", e);
            std::process::exit(1);
        }
    };

    // Get current branch if not specified
    let target_branch = if let Some(b) = branch {
        b
    } else {
        match repo.head() {
            Ok(head) => {
                if head.is_branch() {
                    head.shorthand().unwrap_or_else(|_| "HEAD".to_string())
                } else {
                    "HEAD".to_string()
                }
            }
            Err(_) => "HEAD".to_string(),
        }
    };

    // Get list of commits using git log
    let mut args = repo.global_args_for_exec();
    args.push("log".to_string());
    args.push("--format=%H%n%s".to_string());
    if let Some(lim) = limit {
        args.push(format!("-n{}", lim));
    }
    args.push(target_branch.clone());

    let output = match exec_git(&args) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error getting commit list: {}", e);
            std::process::exit(1);
        }
    };

    let commits_output = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = commits_output.lines().collect();

    // Aggregate statistics
    let mut total_commits = 0;
    let mut commits_with_ai = 0;
    let mut total_ai_lines = 0;
    let mut total_human_lines = 0;
    let mut total_mixed_lines = 0;
    let mut total_additions = 0;

    let mut commit_details = Vec::new();

    // Parse commits (format: hash\nmessage\n)
    let mut i = 0;
    while i < lines.len() {
        let sha = lines[i].trim();
        if sha.is_empty() {
            i += 1;
            continue;
        }

        let message = if i + 1 < lines.len() {
            lines[i + 1].trim().to_string()
        } else {
            "".to_string()
        };
        i += 2;

        let refname = target_branch.clone();
        
        if let Ok(stats) = stats_for_commit_stats(&repo, sha, &refname) {
            total_commits += 1;

            let ai_adds = stats.ai_additions;
            let human_adds = stats.human_additions;
            let mixed_adds = stats.mixed_additions;
            let diff_adds = stats.git_diff_added_lines;

            if ai_adds > 0 {
                commits_with_ai += 1;
            }

            total_ai_lines += ai_adds;
            total_human_lines += human_adds;
            total_mixed_lines += mixed_adds;
            total_additions += diff_adds;

            if json_output {
                let ai_pct = if diff_adds > 0 {
                    ((ai_adds as f64 / diff_adds as f64) * 100.0).round() as u32
                } else {
                    0
                };

                commit_details.push(json!({
                    "sha": &sha[..8.min(sha.len())],
                    "message": message,
                    "ai_percentage": ai_pct,
                    "ai_lines": ai_adds,
                    "human_lines": human_adds,
                    "mixed_lines": mixed_adds,
                    "total_lines": diff_adds,
                }));
            }
        }
    }

    // Calculate overall percentage
    let overall_ai_percentage = if total_additions > 0 {
        ((total_ai_lines as f64 / total_additions as f64) * 100.0).round() as u32
    } else {
        0
    };

    let overall_human_percentage = if total_additions > 0 {
        ((total_human_lines as f64 / total_additions as f64) * 100.0).round() as u32
    } else {
        0
    };

    if json_output {
        let output = json!({
            "summary": {
                "total_commits": total_commits,
                "commits_with_ai": commits_with_ai,
                "ai_percentage": overall_ai_percentage,
                "human_percentage": overall_human_percentage,
                "total_ai_lines": total_ai_lines,
                "total_human_lines": total_human_lines,
                "total_mixed_lines": total_mixed_lines,
                "total_additions": total_additions,
            },
            "commits": commit_details,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        // Pretty printed output
        println!("");
        println!("╔══════════════════════════════════════════════════════════════════════════╗");
        println!("║                                                                          ║");
        println!("║                  🏆 REPOSITORY AI STATISTICS 🏆                          ║");
        println!("║                                                                          ║");
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        println!("║                                                                          ║");
        
        // Big overall AI percentage display
        let ai_display = format!("Overall AI Contribution: {}%", overall_ai_percentage);
        println!("║         {}         ║", format!("\x1b[1;36m{}\x1b[0m", ai_display));
        println!("║                                                                          ║");
        
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        println!("║  Branch: {:<63}║", target_branch);
        println!("║  Total Commits Analyzed: {:<48}║", total_commits);
        println!("║  Commits with AI: {:<55}║", commits_with_ai);
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        println!("║                                                                          ║");
        println!("║  📊 Aggregate Breakdown:                                                 ║");
        println!("║                                                                          ║");
        println!("║    AI Lines:       {:>7} / {:<7} ({:>3}%)                              ║",
            total_ai_lines, total_additions, overall_ai_percentage);
        println!("║    Human Lines:    {:>7} / {:<7} ({:>3}%)                              ║",
            total_human_lines, total_additions, overall_human_percentage);
        println!("║    Mixed Lines:    {:>7}                                                 ║", total_mixed_lines);
        println!("║    Total Lines:    {:>7}                                                 ║", total_additions);
        println!("║                                                                          ║");
        
        // Visual bar
        println!("║  Distribution:                                                           ║");
        let bar_width: usize = 60;
        let ai_bar_width = if total_additions > 0 {
            (total_ai_lines as f64 / total_additions as f64 * bar_width as f64) as usize
        } else {
            0
        };
        let human_bar_width = bar_width.saturating_sub(ai_bar_width);
        
        let ai_bar = "█".repeat(ai_bar_width);
        let human_bar = "█".repeat(human_bar_width);
        
        println!("║    \x1b[1;31m{}\x1b[1;32m{}\x1b[0m     ║", ai_bar, human_bar);
        println!("║    \x1b[1;31mAI\x1b[0m                                                    \x1b[1;32mHuman\x1b[0m        ║");
        println!("║                                                                          ║");
        println!("╚══════════════════════════════════════════════════════════════════════════╝");
        println!("");

        if total_commits == 0 {
            println!("⚠️  No commits found with AI tracking data.");
            println!("   Make sure you've committed code with git-ai-tracker active!");
            println!("");
        } else {
            println!("💡 Tips:");
            println!("   • Use --json for machine-readable output");
            println!("   • Use --limit N to analyze only the last N commits");
            println!("   • Use --branch <name> to analyze a specific branch");
            println!("");
        }
    }
}
