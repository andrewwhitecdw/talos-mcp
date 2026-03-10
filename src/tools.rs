use serde_json::json;

pub fn get_all_tool_schemas() -> Vec<serde_json::Value> {
    vec![
        // System Monitoring Tools
        containers_tool(),
        stats_tool(),
        get_processes_tool(),
        memory_verbose_tool(),
        get_cpu_memory_usage_tool(),
        // File Operations Tools
        list_tool(),
        read_tool(),
        copy_tool(),
        get_usage_tool(),
        get_mounts_tool(),
        // Network Tools
        interfaces_tool(),
        routes_tool(),
        get_netstat_tool(),
        capture_packets_tool(),
        get_network_io_cgroups_tool(),
        list_network_interfaces_tool(),
        // Service & Log Tools
        dmesg_tool(),
        service_tool(),
        restart_tool(),
        get_logs_tool(),
        get_events_tool(),
        // Storage Tools
        disks_tool(),
        list_disks_tool(),
        // Cluster Management Tools
        get_health_tool(),
        get_version_tool(),
        get_time_tool(),
        // Node Management Tools
        reboot_node_tool(),
        shutdown_node_tool(),
        reset_node_tool(),
        upgrade_node_tool(),
        upgrade_k8s_tool(),
        // Configuration Tools
        apply_config_tool(),
        validate_config_tool(),
        // etcd Tools
        get_etcd_status_tool(),
        get_etcd_members_tool(),
        bootstrap_etcd_tool(),
        defrag_etcd_tool(),
    ]
}

// ============ System Monitoring Tools ============

fn containers_tool() -> serde_json::Value {
    json!({
        "name": "containers",
        "description": "List containers running on a Talos node with optional Kubernetes namespace filtering",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "kubernetes": {
                    "type": "boolean",
                    "description": "Show Kubernetes containers (cri namespace)"
                },
                "namespace": {
                    "type": "string",
                    "description": "Container namespace to filter by"
                }
            },
            "required": ["node"]
        }
    })
}

fn stats_tool() -> serde_json::Value {
    json!({
        "name": "stats",
        "description": "Get resource statistics for containers on a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "kubernetes": {
                    "type": "boolean",
                    "description": "Show Kubernetes container stats"
                }
            },
            "required": ["node"]
        }
    })
}

fn get_processes_tool() -> serde_json::Value {
    json!({
        "name": "get_processes",
        "description": "List running processes on a Talos node with sorting capabilities",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "sort": {
                    "type": "string",
                    "enum": ["cpu", "rss", "pid", "time"],
                    "description": "Sort processes by: cpu, rss, pid, or time"
                }
            },
            "required": ["node"]
        }
    })
}

fn memory_verbose_tool() -> serde_json::Value {
    json!({
        "name": "memory_verbose",
        "description": "Get detailed memory information from a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                }
            },
            "required": ["node"]
        }
    })
}

fn get_cpu_memory_usage_tool() -> serde_json::Value {
    json!({
        "name": "get_cpu_memory_usage",
        "description": "Get combined CPU and memory usage statistics from a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                }
            },
            "required": ["node"]
        }
    })
}

// ============ File Operations Tools ============

fn list_tool() -> serde_json::Value {
    json!({
        "name": "list",
        "description": "List files and directories on a Talos node with advanced filtering options",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "path": {
                    "type": "string",
                    "description": "Directory path to list (default: /)"
                },
                "long": {
                    "type": "boolean",
                    "description": "Show detailed file information"
                },
                "humanize": {
                    "type": "boolean",
                    "description": "Show file sizes in human-readable format"
                },
                "recurse": {
                    "type": "boolean",
                    "description": "Recursively list subdirectories"
                },
                "depth": {
                    "type": "integer",
                    "description": "Maximum recursion depth"
                },
                "r#type": {
                    "type": "array",
                    "items": {
                        "type": "string",
                        "enum": ["f", "d", "l"]
                    },
                    "description": "Filter by file type: f (file), d (directory), l (symlink)"
                }
            },
            "required": ["node"]
        }
    })
}

fn read_tool() -> serde_json::Value {
    json!({
        "name": "read",
        "description": "Read file content from a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "path": {
                    "type": "string",
                    "description": "Path to the file to read"
                }
            },
            "required": ["node", "path"]
        }
    })
}

fn copy_tool() -> serde_json::Value {
    json!({
        "name": "copy",
        "description": "Copy files from a Talos node to local system",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "source": {
                    "type": "string",
                    "description": "Source path on the Talos node"
                },
                "destination": {
                    "type": "string",
                    "description": "Destination path on local system"
                }
            },
            "required": ["node", "source", "destination"]
        }
    })
}

fn get_usage_tool() -> serde_json::Value {
    json!({
        "name": "get_usage",
        "description": "Get disk usage information for a path on a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "path": {
                    "type": "string",
                    "description": "Path to check disk usage for"
                }
            },
            "required": ["node"]
        }
    })
}

fn get_mounts_tool() -> serde_json::Value {
    json!({
        "name": "get_mounts",
        "description": "Get filesystem mount details from a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                }
            },
            "required": ["node"]
        }
    })
}

// ============ Network Tools ============

fn interfaces_tool() -> serde_json::Value {
    json!({
        "name": "interfaces",
        "description": "Get detailed network interface information with multiple output formats",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "namespace": {
                    "type": "string",
                    "description": "Network namespace to query"
                },
                "output": {
                    "type": "string",
                    "enum": ["table", "json", "yaml"],
                    "description": "Output format (table, json, or yaml)"
                }
            },
            "required": ["node"]
        }
    })
}

fn routes_tool() -> serde_json::Value {
    json!({
        "name": "routes",
        "description": "Get the routing table from a Talos node with advanced filtering",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "namespace": {
                    "type": "string",
                    "description": "Network namespace to query"
                },
                "output": {
                    "type": "string",
                    "enum": ["table", "json", "yaml"],
                    "description": "Output format (table, json, or yaml)"
                }
            },
            "required": ["node"]
        }
    })
}

fn get_netstat_tool() -> serde_json::Value {
    json!({
        "name": "get_netstat",
        "description": "Get network connection statistics from a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                }
            },
            "required": ["node"]
        }
    })
}

fn capture_packets_tool() -> serde_json::Value {
    json!({
        "name": "capture_packets",
        "description": "Capture network packets on a Talos node interface",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "interface": {
                    "type": "string",
                    "description": "Network interface to capture on"
                },
                "duration": {
                    "type": "integer",
                    "description": "Duration of capture in seconds"
                }
            },
            "required": ["node", "interface"]
        }
    })
}

fn get_network_io_cgroups_tool() -> serde_json::Value {
    json!({
        "name": "get_network_io_cgroups",
        "description": "Get network I/O statistics from cgroups on a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                }
            },
            "required": ["node"]
        }
    })
}

fn list_network_interfaces_tool() -> serde_json::Value {
    json!({
        "name": "list_network_interfaces",
        "description": "Legacy network interface listing for a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                }
            },
            "required": ["node"]
        }
    })
}

// ============ Service & Log Tools ============

fn dmesg_tool() -> serde_json::Value {
    json!({
        "name": "dmesg",
        "description": "Get kernel messages from a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "tail": {
                    "type": "boolean",
                    "description": "Show only recent messages"
                }
            },
            "required": ["node"]
        }
    })
}

fn service_tool() -> serde_json::Value {
    json!({
        "name": "service",
        "description": "Manage system services on a Talos node (start, stop, restart, status)",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "service": {
                    "type": "string",
                    "description": "Name of the service"
                },
                "action": {
                    "type": "string",
                    "enum": ["start", "stop", "restart", "status"],
                    "description": "Action to perform on the service"
                }
            },
            "required": ["node", "service", "action"]
        }
    })
}

fn restart_tool() -> serde_json::Value {
    json!({
        "name": "restart",
        "description": "Restart a service on a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "service": {
                    "type": "string",
                    "description": "Name of the service to restart"
                }
            },
            "required": ["node", "service"]
        }
    })
}

fn get_logs_tool() -> serde_json::Value {
    json!({
        "name": "get_logs",
        "description": "Get logs from a Talos node or service",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "service": {
                    "type": "string",
                    "description": "Service name (e.g., 'kubelet', 'containerd') or 'k8s' for Kubernetes logs"
                },
                "tail": {
                    "type": "integer",
                    "description": "Number of lines to show from the end of the logs (default: all lines)"
                },
                "kubernetes": {
                    "type": "boolean",
                    "description": "Show Kubernetes logs (shortcut for service='k8s')"
                },
                "namespace": {
                    "type": "string",
                    "description": "Kubernetes namespace (for Kubernetes logs)"
                }
            },
            "required": ["node", "service"]
        }
    })
}

fn get_events_tool() -> serde_json::Value {
    json!({
        "name": "get_events",
        "description": "Get system events from a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "tail": {
                    "type": "integer",
                    "description": "Number of events to show"
                }
            },
            "required": ["node"]
        }
    })
}

// ============ Storage Tools ============

fn disks_tool() -> serde_json::Value {
    json!({
        "name": "disks",
        "description": "Get detailed disk information with multiple output formats",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "namespace": {
                    "type": "string",
                    "description": "Namespace to query"
                },
                "output": {
                    "type": "string",
                    "enum": ["table", "json", "yaml"],
                    "description": "Output format (table, json, or yaml)"
                }
            },
            "required": ["node"]
        }
    })
}

fn list_disks_tool() -> serde_json::Value {
    json!({
        "name": "list_disks",
        "description": "List disks on a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                }
            },
            "required": ["node"]
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn validate_tool_schema(tool: &Value) {
        assert!(tool.is_object(), "Tool schema should be an object");
        assert!(tool.get("name").is_some(), "Tool should have 'name' field");
        assert!(
            tool.get("description").is_some(),
            "Tool should have 'description' field"
        );
        let input_schema = tool
            .get("inputSchema")
            .expect("Tool should have 'inputSchema' field");
        assert!(input_schema.is_object(), "inputSchema should be an object");
        assert_eq!(
            input_schema.get("type").unwrap(),
            "object",
            "inputSchema type should be 'object'"
        );
        assert!(
            input_schema.get("properties").is_some(),
            "inputSchema should have 'properties' field"
        );
    }

    #[test]
    fn test_get_all_tool_schemas_returns_tools() {
        let tools = get_all_tool_schemas();
        assert!(!tools.is_empty(), "Should return at least one tool");
        assert!(
            tools.len() > 30,
            "Should return at least 30 tools, got {}",
            tools.len()
        );
    }

    #[test]
    fn test_all_tools_have_valid_schemas() {
        let tools = get_all_tool_schemas();
        for tool in &tools {
            validate_tool_schema(tool);
        }
    }

    #[test]
    fn test_containers_tool_structure() {
        let tool = containers_tool();
        assert_eq!(tool.get("name").unwrap(), "containers");
        let schema = tool.get("inputSchema").unwrap();
        let required = schema.get("required").unwrap().as_array().unwrap();
        assert!(required.contains(&json!("node")), "node should be required");
    }

    #[test]
    fn test_stats_tool_structure() {
        let tool = stats_tool();
        assert_eq!(tool.get("name").unwrap(), "stats");
        validate_tool_schema(&tool);
    }

    #[test]
    fn test_list_tool_structure() {
        let tool = list_tool();
        assert_eq!(tool.get("name").unwrap(), "list");
        validate_tool_schema(&tool);
    }

    #[test]
    fn test_interfaces_tool_structure() {
        let tool = interfaces_tool();
        assert_eq!(tool.get("name").unwrap(), "interfaces");
        validate_tool_schema(&tool);
    }

    #[test]
    fn test_routes_tool_structure() {
        let tool = routes_tool();
        assert_eq!(tool.get("name").unwrap(), "routes");
        validate_tool_schema(&tool);
    }

    #[test]
    fn test_dmesg_tool_structure() {
        let tool = dmesg_tool();
        assert_eq!(tool.get("name").unwrap(), "dmesg");
        validate_tool_schema(&tool);
    }

    #[test]
    fn test_disks_tool_structure() {
        let tool = disks_tool();
        assert_eq!(tool.get("name").unwrap(), "disks");
        validate_tool_schema(&tool);
    }

    #[test]
    fn test_get_health_tool_structure() {
        let tool = get_health_tool();
        assert_eq!(tool.get("name").unwrap(), "get_health");
        validate_tool_schema(&tool);
    }

    #[test]
    fn test_reboot_node_tool_structure() {
        let tool = reboot_node_tool();
        assert_eq!(tool.get("name").unwrap(), "reboot_node");
        validate_tool_schema(&tool);
    }

    #[test]
    fn test_apply_config_tool_structure() {
        let tool = apply_config_tool();
        assert_eq!(tool.get("name").unwrap(), "apply_config");
        let schema = tool.get("inputSchema").unwrap();
        let required = schema.get("required").unwrap().as_array().unwrap();
        assert!(required.contains(&json!("node")), "node should be required");
    }
}

// ============ Cluster Management Tools ============

fn get_health_tool() -> serde_json::Value {
    json!({
        "name": "get_health",
        "description": "Get comprehensive cluster health status with topology support",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of a Talos node (optional, queries cluster if not provided)"
                }
            },
            "required": []
        }
    })
}

fn get_version_tool() -> serde_json::Value {
    json!({
        "name": "get_version",
        "description": "Get Talos version information from a node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "short": {
                    "type": "boolean",
                    "description": "Show compact version format"
                }
            },
            "required": ["node"]
        }
    })
}

fn get_time_tool() -> serde_json::Value {
    json!({
        "name": "get_time",
        "description": "Get time information and NTP synchronization status from a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node (required)"
                },
                "check": {
                    "type": "string",
                    "description": "NTP server to check synchronization against (e.g., 'pool.ntp.org')"
                }
            },
            "required": ["node"]
        }
    })
}

// ============ Node Management Tools ============

fn reboot_node_tool() -> serde_json::Value {
    json!({
        "name": "reboot_node",
        "description": "Safely reboot a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "mode": {
                    "type": "string",
                    "enum": ["default", "powercycle"],
                    "description": "Reboot mode"
                }
            },
            "required": ["node"]
        }
    })
}

fn shutdown_node_tool() -> serde_json::Value {
    json!({
        "name": "shutdown_node",
        "description": "Gracefully shutdown a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "force": {
                    "type": "boolean",
                    "description": "Force shutdown without confirmation"
                }
            },
            "required": ["node"]
        }
    })
}

fn reset_node_tool() -> serde_json::Value {
    json!({
        "name": "reset_node",
        "description": "Perform a factory reset on a Talos node (DANGEROUS)",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "graceful": {
                    "type": "boolean",
                    "description": "Perform graceful reset"
                },
                "reboot": {
                    "type": "boolean",
                    "description": "Reboot after reset"
                }
            },
            "required": ["node"]
        }
    })
}

fn upgrade_node_tool() -> serde_json::Value {
    json!({
        "name": "upgrade_node",
        "description": "Upgrade Talos OS on a node to a new image",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "image": {
                    "type": "string",
                    "description": "New Talos image to upgrade to"
                },
                "preserve": {
                    "type": "boolean",
                    "description": "Preserve data during upgrade"
                },
                "stage": {
                    "type": "boolean",
                    "description": "Stage the upgrade for next reboot"
                }
            },
            "required": ["node", "image"]
        }
    })
}

fn upgrade_k8s_tool() -> serde_json::Value {
    json!({
        "name": "upgrade_k8s",
        "description": "Upgrade Kubernetes version on a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "version": {
                    "type": "string",
                    "description": "Kubernetes version to upgrade to"
                }
            },
            "required": ["node", "version"]
        }
    })
}

// ============ Configuration Tools ============

fn apply_config_tool() -> serde_json::Value {
    json!({
        "name": "apply_config",
        "description": "Apply Talos configuration to a node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "file": {
                    "type": "string",
                    "description": "Path to the configuration file"
                },
                "mode": {
                    "type": "string",
                    "enum": ["auto", "no-reboot", "staged"],
                    "description": "Apply mode"
                }
            },
            "required": ["node", "file"]
        }
    })
}

fn validate_config_tool() -> serde_json::Value {
    json!({
        "name": "validate_config",
        "description": "Validate a Talos configuration file",
        "inputSchema": {
            "type": "object",
            "properties": {
                "file": {
                    "type": "string",
                    "description": "Path to the configuration file to validate"
                },
                "mode": {
                    "type": "string",
                    "description": "Configuration mode (cloud, metal, etc.)"
                }
            },
            "required": ["file"]
        }
    })
}

// ============ etcd Tools ============

fn get_etcd_status_tool() -> serde_json::Value {
    json!({
        "name": "get_etcd_status",
        "description": "Get etcd status for a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "context": {
                    "type": "string",
                    "description": "Kubernetes context to use"
                }
            },
            "required": ["node"]
        }
    })
}

fn get_etcd_members_tool() -> serde_json::Value {
    json!({
        "name": "get_etcd_members",
        "description": "Get etcd members for a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "context": {
                    "type": "string",
                    "description": "Kubernetes context to use"
                }
            },
            "required": ["node"]
        }
    })
}

fn bootstrap_etcd_tool() -> serde_json::Value {
    json!({
        "name": "bootstrap_etcd",
        "description": "Bootstrap etcd on a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "context": {
                    "type": "string",
                    "description": "Kubernetes context to use"
                }
            },
            "required": ["node"]
        }
    })
}

fn defrag_etcd_tool() -> serde_json::Value {
    json!({
        "name": "defrag_etcd",
        "description": "Defragment etcd on a Talos node",
        "inputSchema": {
            "type": "object",
            "properties": {
                "node": {
                    "type": "string",
                    "description": "The IP address or hostname of the Talos node"
                },
                "context": {
                    "type": "string",
                    "description": "Kubernetes context to use"
                }
            },
            "required": ["node"]
        }
    })
}
