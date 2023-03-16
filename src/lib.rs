#[derive(PartialEq, Debug)]
pub enum RunAsMode {
    RunAsLocal,
    RunAsOwner,
    RunAsSudo,
}

#[derive(Debug)]
pub struct RuntimeEnvironment {
    pub run_as_mode: RunAsMode,
    pub run_as_user: String,
    pub work_dir: String,
    pub env_vars: String,
    pub enable_x_flag: bool,    //xtrace  : Write each command to standard error (preceded by a ‘+ ’) before it is executed.  Useful for debugging.
    pub enable_v_flag: bool,    //verbose : The shell writes its input to standard error as it is read.  Useful for debugging.
    pub enable_n_flag: bool,    //noexec  : If not interactive, read commands but do not execute them.  This is useful for checking the syntax of shell scripts.
    pub enable_e_flag: bool,    //errexit 
}

pub mod metrics{

    use anyhow::{Result, anyhow};
    use psutil::process::{Process, ProcessCpuTimes, MemoryInfo};

    // Structure to store process metrics
    pub struct ProcessMetrics {
        pub pid: u32,
        process: Process,
        memory_info: MemoryInfo,
    }

    impl ProcessMetrics {
        // Constructor for ProcessMetrics
        pub fn new(pid: u32) -> Result<Self> {
            let process = Process::new(pid)?;
            let memory_info = process.memory_info()?;
            Ok(ProcessMetrics { pid, process, memory_info })
        }

        // Get CPU usage percentage of the process
        pub fn cpu_percent(&mut self) -> Result<f32> {
            self.process.cpu_percent()
                .map_err(|e| anyhow!("Failed to get CPU percent: {}", e))
        }

        // Get physical memory usage of the process
        pub fn physical_memory(&self) -> u64 {
            self.memory_info.rss()
        }

        // Get virtual memory usage of the process
        pub fn virtual_memory(&self) -> u64 {
            self.memory_info.vms()
        }

        // Get CPU times of the process
        pub fn cpu_times(&mut self) -> Result<ProcessCpuTimes> {
            self.process.cpu_times()
                .map_err(|e| anyhow!("Failed to get CPU times: {}", e))
        }

        // Kill the process
        pub fn kill(&mut self) -> Result<()> {
            self.process.kill()
                .map_err(|e| anyhow!("Failed to kill process: {}", e))
        }
    }

    use sysinfo::{System, SystemExt, RefreshKind};

    // Structure to store system metrics
    pub struct SystemMetrics {
        system: System,
        refreshes: RefreshKind,
    }

    impl SystemMetrics {
        // Constructor for SystemMetrics
        pub fn new() -> Result<Self> {
            let system = System::new_all();
            let refreshes: RefreshKind = RefreshKind::new();
            Ok(SystemMetrics { system, refreshes })
        }

        // Get total memory of the system
        pub fn total_memory(&mut self) -> Result<u64> {
            self.system.refresh_specifics(self.refreshes.with_memory());
            Ok(self.system.total_memory())
        }

        // Get available memory of the system
        pub fn available_memory(&mut self) -> Result<u64> {
            self.system.refresh_specifics(self.refreshes.with_memory());
            Ok(self.system.available_memory())
        }

        // Get used memory of the system
        pub fn used_memory(&mut self) -> Result<u64> {
            self.system.refresh_specifics(self.refreshes.with_memory());
            let total = self.total_memory()?;
            let available = self.available_memory()?;
            Ok(total - available)
        }
    }
   

}

///Submit a process
#[inline]
pub fn submit(run_env:RuntimeEnvironment) {
    
    //let user_credentials:Option<(&str, &str)>;
    //let user_name: &str;
    //let user_passwd: &str;
    if run_env.run_as_mode != RunAsMode::RunAsLocal {
        /* locate data for authentication
        user_credentials = get_user_credentials(&run_env.run_as_user);
        if user_credentials == None {
            println!("Mo se ha podido ontener la credencial");
            //Break code
        }
         */
    }

    let os_family = std::env::consts::FAMILY;

    if os_family == "unix" {
        let run_as_prefix = match run_env.run_as_mode {
            RunAsMode::RunAsOwner   => String::from(""), //format!("echo {} | su {} -p -c", user_passwd, user_name),
            RunAsMode::RunAsSudo    => String::from(""), //format!("echo {} | sudo -S", user_passwd),
            _  => String::from(""),
        };

        println!("{}", run_as_prefix);
       //Ok(());

    } else if os_family == "windows" {

    } else {
        println!("Unknow OS Family");
    }
    
    // # RunAs user1
    // $ echo pass1 | su user1 -p -c "ls -ltr /home/"
    // $ echo Dan13lDan13l | su daniel -p -c "ls -ltr /home/;otro_comando"

    // #RunAs user1 like superuser (sudoers requiered)
    // $ echo passuser1 | sudo -S ls -ltr
    // $ echo rHernandez101 | sudo -S sh -c 'set -x;ls -ltr;id'

    /*
    #!/bin/sh
    sudo su - username << block
    cd /; 
    tail filename;
    block

    sudo -s -- <<EOF
    id
    pwd
    echo "Done."
    EOF


    */

    //Win
    //$ runas /user:user1 /PASSWORD:contrasenia dir

}


