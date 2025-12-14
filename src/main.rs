use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post};
use serde::{Serialize, Deserialize};
use sysinfo::{System, Disks, Networks};
use chrono::{DateTime, Local};
use std::process::Command;
use std::env;
use std::ffi::OsStr;

// 定义 API 响应结构
#[derive(Serialize, Debug)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
    timestamp: String,
}

// 系统信息结构
#[derive(Serialize, Debug)]
struct SystemInfo {
    os: String,
    hostname: String,
    kernel_version: String,
    uptime: u64,
    boot_time: u64,
    current_user: String,
}

// CPU 信息结构
#[derive(Serialize, Debug, Clone)]
struct CpuInfo {
    name: String,
    vendor_id: String,
    brand: String,
    frequency: u64,
    usage: f32,
    cores: usize,
    load_average: LoadAverage,
}

#[derive(Serialize, Debug, Clone)]
struct LoadAverage {
    one_min: f64,
    five_min: f64,
    fifteen_min: f64,
}

// 内存信息结构
#[derive(Serialize, Debug)]
struct MemoryInfo {
    total_memory: u64,
    used_memory: u64,
    free_memory: u64,
    total_swap: u64,
    used_swap: u64,
    free_swap: u64,
    memory_percent: f32,
}

// 磁盘信息结构
#[derive(Serialize, Debug)]
struct DiskInfo {
    name: String,
    file_system: String,
    total_space: u64,
    available_space: u64,
    used_space: u64,
    mount_point: String,
    is_removable: bool,
}

// 网络信息结构
#[derive(Serialize, Debug)]
struct NetworkInfo {
    name: String,
    mac_address: String,
    received_bytes: u64,
    transmitted_bytes: u64,
    packets_received: u64,
    packets_transmitted: u64,
    total_received: u64,
    total_transmitted: u64,
}

// 进程信息结构
#[derive(Serialize, Debug)]
struct ProcessInfo {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
    status: String,
    run_time: u64,
    command: Vec<String>,
}

// 完整系统报告
#[derive(Serialize, Debug)]
struct FullSystemReport {
    system: SystemInfo,
    cpu: Vec<CpuInfo>,
    memory: MemoryInfo,
    disks: Vec<DiskInfo>,
    networks: Vec<NetworkInfo>,
    processes: Vec<ProcessInfo>,
    timestamp: String,
}

// 用于接收 POST 请求的结构
#[derive(Deserialize)]
struct ProcessQuery {
    name: Option<String>,
    limit: Option<usize>,
}

// 辅助函数：获取当前时间戳
fn get_timestamp() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

// 辅助函数：API 响应包装
fn api_response<T: Serialize>(success: bool, message: &str, data: Option<T>) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        success,
        message: message.to_string(),
        data,
        timestamp: get_timestamp(),
    })
}

// 辅助函数：将 OsStr 转换为 String（跨平台兼容）
fn os_str_to_string(os_str: &OsStr) -> String {
    os_str.to_string_lossy().to_string()
}

// 1. 欢迎页面
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
    <html>
        <head>
            <title>System Info API</title>
            <meta charset="UTF-8">
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                h1 { color: #333; }
                .endpoint { background: #f5f5f5; padding: 10px; margin: 10px 0; border-left: 4px solid #007acc; }
                code { background: #eee; padding: 2px 4px; border-radius: 3px; }
            </style>
        </head>
        <body>
            <h1>🖥️ System Information API</h1>
            <p>Rust 实现的系统信息监控 API</p>
            
            <h2>可用端点：</h2>
            <div class="endpoint">
                <strong>GET /</strong> - 此帮助页面
            </div>
            <div class="endpoint">
                <strong>GET /api/health</strong> - 健康检查
            </div>
            <div class="endpoint">
                <strong>GET /api/system</strong> - 系统信息
            </div>
            <div class="endpoint">
                <strong>GET /api/cpu</strong> - CPU 信息
            </div>
            <div class="endpoint">
                <strong>GET /api/memory</strong> - 内存信息
            </div>
            <div class="endpoint">
                <strong>GET /api/disks</strong> - 磁盘信息
            </div>
            <div class="endpoint">
                <strong>GET /api/networks</strong> - 网络信息
            </div>
            <div class="endpoint">
                <strong>GET /api/processes</strong> - 进程列表
            </div>
            <div class="endpoint">
                <strong>POST /api/processes/search</strong> - 搜索进程 (JSON body: {"name": "chrome", "limit": 10})
            </div>
            <div class="endpoint">
                <strong>GET /api/full-report</strong> - 完整系统报告
            </div>
            <div class="endpoint">
                <strong>GET /api/env</strong> - 环境变量
            </div>
            <div class="endpoint">
                <strong>POST /api/execute</strong> - 执行系统命令
            </div>
            
            <h2>使用示例：</h2>
            <pre><code>curl http://localhost:8080/api/system
curl http://localhost:8080/api/cpu</code></pre>
        </body>
    </html>
    "#)
}

// 2. 健康检查端点
#[get("/api/health")]
async fn health_check() -> impl Responder {
    api_response(true, "API 运行正常", Some("System Info API is running"))
}

// 3. 获取系统信息
#[get("/api/system")]
async fn get_system_info() -> impl Responder {
    let info = SystemInfo {
        os: System::name().unwrap_or_else(|| "Unknown".to_string()),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        uptime: System::uptime(),
        boot_time: System::boot_time(),
        current_user: whoami::username(),
    };
    
    api_response(true, "系统信息获取成功", Some(info))
}

// 4. 获取 CPU 信息
#[get("/api/cpu")]
async fn get_cpu_info() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    
    let load_avg = System::load_average();
    let load_average = LoadAverage {
        one_min: load_avg.one,
        five_min: load_avg.five,
        fifteen_min: load_avg.fifteen,
    };
    
    let cpus: Vec<CpuInfo> = sys.cpus().iter().map(|cpu| {
        CpuInfo {
            name: cpu.name().to_string(),
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
            usage: cpu.cpu_usage(),
            cores: sys.physical_core_count().unwrap_or(0),
            load_average: load_average.clone(),
        }
    }).collect();
    
    api_response(true, "CPU 信息获取成功", Some(cpus))
}

// 5. 获取内存信息
#[get("/api/memory")]
async fn get_memory_info() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let free_memory = sys.free_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let free_swap = sys.free_swap();
    
    let memory_percent = if total_memory > 0 {
        (used_memory as f32 / total_memory as f32) * 100.0
    } else {
        0.0
    };
    
    let info = MemoryInfo {
        total_memory,
        used_memory,
        free_memory,
        total_swap,
        used_swap,
        free_swap,
        memory_percent,
    };
    
    api_response(true, "内存信息获取成功", Some(info))
}

// 6. 获取磁盘信息
#[get("/api/disks")]
async fn get_disk_info() -> impl Responder {
    let disks = Disks::new_with_refreshed_list();
    
    let disk_info: Vec<DiskInfo> = disks.list().iter().map(|disk| {
        DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            file_system: os_str_to_string(disk.file_system()),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            used_space: disk.total_space() - disk.available_space(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            is_removable: disk.is_removable(),
        }
    }).collect();
    
    api_response(true, "磁盘信息获取成功", Some(disk_info))
}

// 7. 获取网络信息
#[get("/api/networks")]
async fn get_network_info() -> impl Responder {
    let networks = Networks::new_with_refreshed_list();
    
    let network_info: Vec<NetworkInfo> = networks.iter().map(|(name, data)| {
        NetworkInfo {
            name: name.to_string(),
            mac_address: data.mac_address().to_string(),
            received_bytes: data.received(),
            transmitted_bytes: data.transmitted(),
            packets_received: data.packets_received(),
            packets_transmitted: data.packets_transmitted(),
            total_received: data.total_received(),
            total_transmitted: data.total_transmitted(),
        }
    }).collect();
    
    api_response(true, "网络信息获取成功", Some(network_info))
}

// 8. 获取进程列表
#[get("/api/processes")]
async fn get_processes() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_processes();
    
    let processes: Vec<ProcessInfo> = sys.processes().iter()
        .map(|(pid, process)| {
            ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                status: format!("{:?}", process.status()),
                run_time: process.run_time(),
                command: process.cmd().to_vec(),
            }
        })
        .collect();
    
    api_response(true, "进程列表获取成功", Some(processes))
}

// 9. 搜索进程 (POST 请求)
#[post("/api/processes/search")]
async fn search_processes(query: web::Json<ProcessQuery>) -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_processes();
    
    let mut processes: Vec<ProcessInfo> = sys.processes().iter()
        .map(|(pid, process)| {
            ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                status: format!("{:?}", process.status()),
                run_time: process.run_time(),
                command: process.cmd().to_vec(),
            }
        })
        .collect();
    
    // 按进程名过滤
    if let Some(name) = &query.name {
        processes.retain(|p| p.name.to_lowercase().contains(&name.to_lowercase()));
    }
    
    // 限制返回数量
    let limit = query.limit.unwrap_or(50);
    processes.truncate(limit);
    
    api_response(true, &format!("找到 {} 个进程", processes.len()), Some(processes))
}

// 10. 获取完整系统报告
#[get("/api/full-report")]
async fn get_full_report() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // 获取负载
    let load_avg = System::load_average();
    let load_average = LoadAverage {
        one_min: load_avg.one,
        five_min: load_avg.five,
        fifteen_min: load_avg.fifteen,
    };
    
    // CPU 信息
    let cpus: Vec<CpuInfo> = sys.cpus().iter().map(|cpu| {
        CpuInfo {
            name: cpu.name().to_string(),
            vendor_id: cpu.vendor_id().to_string(),
            brand: cpu.brand().to_string(),
            frequency: cpu.frequency(),
            usage: cpu.cpu_usage(),
            cores: sys.physical_core_count().unwrap_or(0),
            load_average: load_average.clone(),
        }
    }).collect();
    
    // 系统信息
    let system_info = SystemInfo {
        os: System::name().unwrap_or_else(|| "Unknown".to_string()),
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        uptime: System::uptime(),
        boot_time: System::boot_time(),
        current_user: whoami::username(),
    };
    
    // 内存信息
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_percent = if total_memory > 0 {
        (used_memory as f32 / total_memory as f32) * 100.0
    } else {
        0.0
    };
    
    let memory_info = MemoryInfo {
        total_memory,
        used_memory,
        free_memory: sys.free_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
        free_swap: sys.free_swap(),
        memory_percent,
    };
    
    // 磁盘信息（使用单独的 Disks 对象）
    let disks = Disks::new_with_refreshed_list();
    let disk_info: Vec<DiskInfo> = disks.list().iter().map(|disk| {
        DiskInfo {
            name: disk.name().to_string_lossy().to_string(),
            file_system: os_str_to_string(disk.file_system()),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
            used_space: disk.total_space() - disk.available_space(),
            mount_point: disk.mount_point().to_string_lossy().to_string(),
            is_removable: disk.is_removable(),
        }
    }).collect();
    
    // 网络信息（使用单独的 Networks 对象）
    let networks = Networks::new_with_refreshed_list();
    let network_info: Vec<NetworkInfo> = networks.iter().map(|(name, data)| {
        NetworkInfo {
            name: name.to_string(),
            mac_address: data.mac_address().to_string(),
            received_bytes: data.received(),
            transmitted_bytes: data.transmitted(),
            packets_received: data.packets_received(),
            packets_transmitted: data.packets_transmitted(),
            total_received: data.total_received(),
            total_transmitted: data.total_transmitted(),
        }
    }).collect();
    
    // 进程信息（限制前20个）
    let processes: Vec<ProcessInfo> = sys.processes().iter()
        .take(20)
        .map(|(pid, process)| {
            ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                status: format!("{:?}", process.status()),
                run_time: process.run_time(),
                command: process.cmd().to_vec(),
            }
        })
        .collect();
    
    let report = FullSystemReport {
        system: system_info,
        cpu: cpus,
        memory: memory_info,
        disks: disk_info,
        networks: network_info,
        processes,
        timestamp: get_timestamp(),
    };
    
    api_response(true, "完整系统报告生成成功", Some(report))
}

// 11. 获取系统环境变量
#[get("/api/env")]
async fn get_env_vars() -> impl Responder {
    let env_vars: Vec<(String, String)> = env::vars().collect();
    api_response(true, "环境变量获取成功", Some(env_vars))
}

// 12. 执行系统命令 (需要谨慎使用，仅演示)
#[post("/api/execute")]
async fn execute_command() -> impl Responder {
    // 注意：实际生产环境中应该限制可执行的命令
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo Safe command executed && ver"])
            .output()
            .expect("执行命令失败")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo 'Safe command executed'; uname -a")
            .output()
            .expect("执行命令失败")
    };
    
    let result = String::from_utf8_lossy(&output.stdout).to_string();
    let error = String::from_utf8_lossy(&output.stderr).to_string();
    
    let response = serde_json::json!({
        "stdout": result,
        "stderr": error,
        "exit_code": output.status.code().unwrap_or(-1),
    });
    
    api_response(true, "命令执行完成", Some(response))
}

// 主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 启动 System Info API 服务器...");
    println!("📡 服务器运行在: http://localhost:8080");
    println!("📖 访问 http://localhost:8080 查看 API 文档");
    println!("🛑 按 Ctrl+C 停止服务器\n");
    
    HttpServer::new(|| {
        App::new()
            // 注册所有路由
            .service(index)
            .service(health_check)
            .service(get_system_info)
            .service(get_cpu_info)
            .service(get_memory_info)
            .service(get_disk_info)
            .service(get_network_info)
            .service(get_processes)
            .service(search_processes)
            .service(get_full_report)
            .service(get_env_vars)
            .service(execute_command)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}