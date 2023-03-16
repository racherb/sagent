use sagent::metrics::ProcessMetrics;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_metrics() {
        // Get the pid of the current process
        //let pid = std::process::id();
        let mut process_metrics = ProcessMetrics::new(49506).unwrap();

        // Ensure that the PID is set correctly
        //assert_eq!(process_metrics.pid, pid);

        // Ensure that we can get the CPU percent
        let cpu_percent = process_metrics.cpu_percent().unwrap();
        assert!(cpu_percent >= 0.0 && cpu_percent <= 100.0);

        // Ensure that we can get the physical and virtual memory usage
        let physical_memory = process_metrics.physical_memory();
        let virtual_memory = process_metrics.virtual_memory();
        assert!(physical_memory > 0);
        assert!(virtual_memory > 0);

        // Ensure that we can get the CPU times
        //let cpu_times = process_metrics.cpu_times().unwrap();
        //assert!(cpu_times.user > 0);
        //assert!(cpu_times.system > 0);

        // Ensure that we can kill the process
        let result = process_metrics.kill();
        assert!(result.is_ok());
    }

    use sagent::metrics::SystemMetrics;

    #[test]
    fn test_system_metrics() {
        let mut system_metrics = SystemMetrics::new().unwrap();

        // Ensure that we can get the total, available, and used memory
        let total_memory = system_metrics.total_memory().unwrap();
        let available_memory = system_metrics.available_memory().unwrap();
        let used_memory = system_metrics.used_memory().unwrap();
        assert!(total_memory > 0);
        assert!(available_memory > 0);
        assert!(used_memory <= total_memory);
    }
}
