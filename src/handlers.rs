use crate::mcp::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use anyhow::{Context, Result};
use std::process::Command;
use tracing::{debug, info};

pub fn handle_tool_call(request: &JsonRpcRequest) -> JsonRpcResponse {
    let params = match &request.params {
        Some(p) => p,
        None => {
            return JsonRpcResponse::error(
                request.id.clone(),
                JsonRpcError::invalid_params("Missing params"),
            );
        }
    };

    let tool_name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
    let arguments = params
        .get("arguments")
        .cloned()
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

    info!("Tool call: {} with args: {:?}", tool_name, arguments);

    let result = match tool_name {
        "containers" => handle_containers(&arguments),
        "stats" => handle_stats(&arguments),
        "get_processes" => handle_get_processes(&arguments),
        "memory_verbose" => handle_memory_verbose(&arguments),
        "get_cpu_memory_usage" => handle_get_cpu_memory_usage(&arguments),
        "list" => handle_list(&arguments),
        "read" => handle_read(&arguments),
        "copy" => handle_copy(&arguments),
        "get_usage" => handle_get_usage(&arguments),
        "get_mounts" => handle_get_mounts(&arguments),
        "interfaces" => handle_interfaces(&arguments),
        "routes" => handle_routes(&arguments),
        "get_netstat" => handle_get_netstat(&arguments),
        "capture_packets" => handle_capture_packets(&arguments),
        "get_network_io_cgroups" => handle_get_network_io_cgroups(&arguments),
        "list_network_interfaces" => handle_list_network_interfaces(&arguments),
        "dmesg" => handle_dmesg(&arguments),
        "service" => handle_service(&arguments),
        "restart" => handle_restart(&arguments),
        "get_logs" => handle_get_logs(&arguments),
        "get_events" => handle_get_events(&arguments),
        "disks" => handle_disks(&arguments),
        "list_disks" => handle_list_disks(&arguments),
        "get_health" => handle_get_health(&arguments),
        "get_version" => handle_get_version(&arguments),
        "get_time" => handle_get_time(&arguments),
        "reboot_node" => handle_reboot_node(&arguments),
        "shutdown_node" => handle_shutdown_node(&arguments),
        "reset_node" => handle_reset_node(&arguments),
        "upgrade_node" => handle_upgrade_node(&arguments),
        "upgrade_k8s" => handle_upgrade_k8s(&arguments),
        "apply_config" => handle_apply_config(&arguments),
        "validate_config" => handle_validate_config(&arguments),
        "get_etcd_status" => handle_get_etcd_status(&arguments),
        "get_etcd_members" => handle_get_etcd_members(&arguments),
        "bootstrap_etcd" => handle_bootstrap_etcd(&arguments),
        "defrag_etcd" => handle_defrag_etcd(&arguments),
        _ => {
            return JsonRpcResponse::error(
                request.id.clone(),
                JsonRpcError::tool_error(&format!("Unknown tool: {}", tool_name)),
            )
        }
    };

    match result {
        Ok(output) => JsonRpcResponse::success(
            request.id.clone(),
            serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": output
                }]
            }),
        ),
        Err(e) => JsonRpcResponse::success(
            request.id.clone(),
            serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Error: {}", e)
                }],
                "isError": true
            }),
        ),
    }
}

fn get_required_string(args: &serde_json::Value, key: &str) -> Result<String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .with_context(|| format!("Missing required parameter: {}", key))
}

fn get_optional_string(args: &serde_json::Value, key: &str) -> Option<String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn get_optional_bool(args: &serde_json::Value, key: &str) -> bool {
    args.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
}

fn get_optional_int(args: &serde_json::Value, key: &str) -> Option<i32> {
    args.get(key).and_then(|v| v.as_i64()).map(|i| i as i32)
}

fn add_context_flag(args: &serde_json::Value, cmd_args: &mut Vec<String>) {
    if let Some(ctx) = get_optional_string(args, "context") {
        cmd_args.push("--context".to_string());
        cmd_args.push(ctx);
    }
}

fn add_context_to_args(context: &Option<String>, cmd_args: &mut Vec<String>) {
    if let Some(ctx) = context {
        cmd_args.push("--context".to_string());
        cmd_args.push(ctx.clone());
    }
}

fn run_talosctl(args: Vec<String>) -> Result<String> {
    debug!("Running: talosctl {}", args.join(" "));

    let output = Command::new("talosctl")
        .args(&args)
        .output()
        .context("Failed to execute talosctl")?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        anyhow::bail!(
            "talosctl failed: {}",
            if stderr.is_empty() { &stdout } else { &stderr }
        );
    }

    Ok(if stdout.is_empty() { stderr } else { stdout })
}

// System Monitoring Handlers

fn handle_containers(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "containers".to_string()];

    add_context_flag(args, &mut cmd_args);

    if get_optional_bool(args, "kubernetes") {
        cmd_args.push("--kubernetes".to_string());
    }

    if let Some(ns) = get_optional_string(args, "namespace") {
        cmd_args.push("--namespace".to_string());
        cmd_args.push(ns);
    }

    run_talosctl(cmd_args)
}

fn handle_stats(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "stats".to_string()];

    add_context_flag(args, &mut cmd_args);

    if get_optional_bool(args, "kubernetes") {
        cmd_args.push("--kubernetes".to_string());
    }

    run_talosctl(cmd_args)
}

fn handle_get_processes(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "processes".to_string()];

    add_context_flag(args, &mut cmd_args);

    if let Some(sort) = get_optional_string(args, "sort") {
        cmd_args.push("--sort".to_string());
        cmd_args.push(sort);
    }

    run_talosctl(cmd_args)
}

fn handle_memory_verbose(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec![
        "--nodes".to_string(),
        node,
        "memory".to_string(),
        "--verbose".to_string(),
    ];
    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_get_cpu_memory_usage(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let context = get_optional_string(args, "context");

    let mut cpu_args = vec!["--nodes".to_string(), node.clone(), "cpu".to_string()];
    add_context_to_args(&context, &mut cpu_args);
    let cpu = run_talosctl(cpu_args)?;

    let mut mem_args = vec!["--nodes".to_string(), node, "memory".to_string()];
    add_context_to_args(&context, &mut mem_args);
    let memory = run_talosctl(mem_args)?;

    Ok(format!(
        "=== CPU ===\n{}\n\n=== Memory ===\n{}",
        cpu.trim(),
        memory.trim()
    ))
}

// File Operations Handlers

fn handle_list(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let path = get_optional_string(args, "path").unwrap_or_else(|| "/".to_string());
    let mut cmd_args = vec!["--nodes".to_string(), node, "ls".to_string(), path];

    add_context_flag(args, &mut cmd_args);

    if get_optional_bool(args, "long") {
        cmd_args.push("-l".to_string());
    }

    if get_optional_bool(args, "humanize") {
        cmd_args.push("-H".to_string());
    }

    if get_optional_bool(args, "recurse") {
        cmd_args.push("-R".to_string());
    }

    if let Some(depth) = get_optional_int(args, "depth") {
        cmd_args.push("--depth".to_string());
        cmd_args.push(depth.to_string());
    }

    if let Some(types) = args.get("type") {
        if let Some(type_arr) = types.as_array() {
            for t in type_arr {
                if let Some(t_str) = t.as_str() {
                    cmd_args.push("-t".to_string());
                    cmd_args.push(t_str.to_string());
                }
            }
        }
    }

    run_talosctl(cmd_args)
}

fn handle_read(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let path = get_required_string(args, "path")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "read".to_string(), path];
    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_copy(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let source = get_required_string(args, "source")?;
    let destination = get_required_string(args, "destination")?;
    let src_path = format!("{}:{}", node, source);
    let mut cmd_args = vec!["cp".to_string(), src_path, destination];
    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_get_usage(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let path = get_optional_string(args, "path").unwrap_or_else(|| "/".to_string());
    let mut cmd_args = vec!["--nodes".to_string(), node, "usage".to_string(), path];
    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_get_mounts(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "mounts".to_string()];
    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

// Network Handlers

fn handle_interfaces(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec![
        "--nodes".to_string(),
        node,
        "get".to_string(),
        "interfaces".to_string(),
    ];

    if let Some(ns) = get_optional_string(args, "namespace") {
        cmd_args.push("--namespace".to_string());
        cmd_args.push(ns);
    }

    if let Some(output) = get_optional_string(args, "output") {
        cmd_args.push("-o".to_string());
        cmd_args.push(output);
    }

    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_routes(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec![
        "--nodes".to_string(),
        node,
        "get".to_string(),
        "routes".to_string(),
    ];

    if let Some(ns) = get_optional_string(args, "namespace") {
        cmd_args.push("--namespace".to_string());
        cmd_args.push(ns);
    }

    if let Some(output) = get_optional_string(args, "output") {
        cmd_args.push("-o".to_string());
        cmd_args.push(output);
    }

    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_get_netstat(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "netstat".to_string()];
    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_capture_packets(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let interface = get_required_string(args, "interface")?;
    let mut cmd_args = vec![
        "--nodes".to_string(),
        node,
        "pcap".to_string(),
        "-i".to_string(),
        interface,
    ];

    if let Some(duration) = get_optional_int(args, "duration") {
        cmd_args.push("--duration".to_string());
        cmd_args.push(duration.to_string());
    }

    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_get_network_io_cgroups(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec![
        "--nodes".to_string(),
        node,
        "cgroups".to_string(),
        "--type".to_string(),
        "network".to_string(),
    ];
    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_list_network_interfaces(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "interfaces".to_string()];
    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

// Service & Log Handlers

fn handle_dmesg(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "dmesg".to_string()];
    add_context_flag(args, &mut cmd_args);

    if get_optional_bool(args, "tail") {
        cmd_args.push("--tail".to_string());
    }

    run_talosctl(cmd_args)
}

fn handle_service(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let service = get_required_string(args, "service")?;
    let action = get_required_string(args, "action")?;

    let mut cmd_args = match action.as_str() {
        "start" => vec![
            "--nodes".to_string(),
            node,
            "service".to_string(),
            service,
            "start".to_string(),
        ],
        "stop" => vec![
            "--nodes".to_string(),
            node,
            "service".to_string(),
            service,
            "stop".to_string(),
        ],
        "restart" => vec![
            "--nodes".to_string(),
            node,
            "service".to_string(),
            service,
            "restart".to_string(),
        ],
        "status" => vec![
            "--nodes".to_string(),
            node,
            "service".to_string(),
            service,
            "status".to_string(),
        ],
        _ => anyhow::bail!("Invalid service action: {}", action),
    };
    add_context_flag(args, &mut cmd_args);

    run_talosctl(cmd_args)
}

fn handle_restart(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let service = get_required_string(args, "service")?;
    let mut cmd_args = vec![
        "--nodes".to_string(),
        node,
        "service".to_string(),
        service,
        "restart".to_string(),
    ];
    add_context_flag(args, &mut cmd_args);
    run_talosctl(cmd_args)
}

fn handle_get_logs(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let service = get_required_string(args, "service")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "logs".to_string(), service];
    add_context_flag(args, &mut cmd_args);

    if let Some(tail) = get_optional_int(args, "tail") {
        cmd_args.push("--tail".to_string());
        cmd_args.push(tail.to_string());
    }

    if get_optional_bool(args, "kubernetes") {
        cmd_args.push("-k".to_string());
    }

    if let Some(ns) = get_optional_string(args, "namespace") {
        cmd_args.push("-n".to_string());
        cmd_args.push(ns);
    }

    run_talosctl(cmd_args)
}

fn handle_get_events(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "events".to_string()];
    add_context_flag(args, &mut cmd_args);

    if let Some(tail) = get_optional_int(args, "tail") {
        cmd_args.push("--tail".to_string());
        cmd_args.push(tail.to_string());
    }

    run_talosctl(cmd_args)
}

// Storage Handlers

fn handle_disks(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec![
        "--nodes".to_string(),
        node,
        "get".to_string(),
        "disks".to_string(),
    ];
    add_context_flag(args, &mut cmd_args);

    if let Some(ns) = get_optional_string(args, "namespace") {
        cmd_args.push("--namespace".to_string());
        cmd_args.push(ns);
    }

    if let Some(output) = get_optional_string(args, "output") {
        cmd_args.push("-o".to_string());
        cmd_args.push(output);
    }

    run_talosctl(cmd_args)
}

fn handle_list_disks(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    run_talosctl(vec!["--nodes".to_string(), node, "disks".to_string()])
}

// Cluster Management Handlers

fn handle_get_health(args: &serde_json::Value) -> Result<String> {
    let mut cmd_args = vec!["health".to_string()];

    if let Some(node) = get_optional_string(args, "node") {
        cmd_args.push("--nodes".to_string());
        cmd_args.push(node);
    }

    run_talosctl(cmd_args)
}

fn handle_get_version(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "version".to_string()];

    if get_optional_bool(args, "short") {
        cmd_args.push("--short".to_string());
    }

    run_talosctl(cmd_args)
}

fn handle_get_time(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "time".to_string()];

    if let Some(check) = get_optional_string(args, "check") {
        cmd_args.push("--check".to_string());
        cmd_args.push(check);
    }

    run_talosctl(cmd_args)
}

// Node Management Handlers

fn handle_reboot_node(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "reboot".to_string()];

    if let Some(mode) = get_optional_string(args, "mode") {
        cmd_args.push("--mode".to_string());
        cmd_args.push(mode);
    }

    run_talosctl(cmd_args)
}

fn handle_shutdown_node(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "shutdown".to_string()];

    if get_optional_bool(args, "force") {
        cmd_args.push("--force".to_string());
    }

    run_talosctl(cmd_args)
}

fn handle_reset_node(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let mut cmd_args = vec!["--nodes".to_string(), node, "reset".to_string()];

    if get_optional_bool(args, "graceful") {
        cmd_args.push("--graceful".to_string());
    }

    if get_optional_bool(args, "reboot") {
        cmd_args.push("--reboot".to_string());
    }

    run_talosctl(cmd_args)
}

fn handle_upgrade_node(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let image = get_required_string(args, "image")?;
    let mut cmd_args = vec![
        "--nodes".to_string(),
        node,
        "upgrade".to_string(),
        "--image".to_string(),
        image,
    ];

    if get_optional_bool(args, "preserve") {
        cmd_args.push("--preserve".to_string());
    }

    if get_optional_bool(args, "stage") {
        cmd_args.push("--stage".to_string());
    }

    run_talosctl(cmd_args)
}

fn handle_upgrade_k8s(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let version = get_required_string(args, "version")?;
    run_talosctl(vec![
        "--nodes".to_string(),
        node,
        "upgrade-k8s".to_string(),
        "--to".to_string(),
        version,
    ])
}

// Configuration Handlers

fn handle_apply_config(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let file = get_required_string(args, "file")?;
    let mut cmd_args = vec![
        "--nodes".to_string(),
        node,
        "apply-config".to_string(),
        "--file".to_string(),
        file,
    ];

    if let Some(mode) = get_optional_string(args, "mode") {
        cmd_args.push("--mode".to_string());
        cmd_args.push(mode);
    }

    run_talosctl(cmd_args)
}

fn handle_validate_config(args: &serde_json::Value) -> Result<String> {
    let file = get_required_string(args, "file")?;
    let mut cmd_args = vec!["validate-config".to_string(), "--file".to_string(), file];

    if let Some(mode) = get_optional_string(args, "mode") {
        cmd_args.push("--mode".to_string());
        cmd_args.push(mode);
    }

    run_talosctl(cmd_args)
}

// etcd Handlers

fn handle_get_etcd_status(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let context = get_optional_string(args, "context");
    let mut cmd_args = vec!["--nodes".to_string(), node];
    add_context_to_args(&context, &mut cmd_args);
    cmd_args.extend(["etcd".to_string(), "status".to_string()]);
    run_talosctl(cmd_args)
}

fn handle_get_etcd_members(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let context = get_optional_string(args, "context");
    let mut cmd_args = vec!["--nodes".to_string(), node];
    add_context_to_args(&context, &mut cmd_args);
    cmd_args.extend(["etcd".to_string(), "members".to_string()]);
    run_talosctl(cmd_args)
}

fn handle_bootstrap_etcd(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let context = get_optional_string(args, "context");
    let mut cmd_args = vec!["--nodes".to_string(), node];
    add_context_to_args(&context, &mut cmd_args);
    cmd_args.push("bootstrap".to_string());
    run_talosctl(cmd_args)
}

fn handle_defrag_etcd(args: &serde_json::Value) -> Result<String> {
    let node = get_required_string(args, "node")?;
    let context = get_optional_string(args, "context");
    let mut cmd_args = vec!["--nodes".to_string(), node];
    add_context_to_args(&context, &mut cmd_args);
    cmd_args.extend(["etcd".to_string(), "defrag".to_string()]);
    run_talosctl(cmd_args)
}
