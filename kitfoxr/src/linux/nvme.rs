use kitfoxi;
use kitfoxm::actions::{
    CriticalBits, Health, HealthArgs, HealthDisposition, HealthResult, HousekeepingTasks,
    Identifier, IdentifierArgs, IdentifierResult, Identify, IdentifyArgs, IdentifyResult, Resource,
    ResourceIdentifier,
};
use kitfoxm_derive::Resource;

pub fn scan() -> Vec<Device> {
    let recipes = kitfoxi::linux::nvme::scan();

    let mut res: Vec<Device> = Vec::new();
    for recipe in recipes {
        res.push(Device { i: recipe });
    }

    res
}

#[derive(Resource, Debug)]
pub struct Device {
    i: kitfoxi::linux::nvme::DeviceRecipe,
}

impl Identifier for Device {
    fn identifier(&self, _: &IdentifierArgs) -> IdentifierResult {
        let mut res: IdentifierResult = Default::default();

        if let Some(s) = self.i.path.to_str() {
            res.identifier = Some(Ok(ResourceIdentifier(String::from(s))));
        }

        res
    }
}

impl Identify for Device {
    fn identify(&self, _args: &IdentifyArgs) -> IdentifyResult {
        let mut res: IdentifyResult = Default::default();

        if let Some(s) = self.i.path.to_str() {
            res.name = Some(Ok(String::from(s)));
        }

        let dev = self.i.make_device();

        let Ok(buffer) = dev.identify_controller() else {
            return res;
        };

        if let Ok(serial) = buffer.serial() {
            res.serial = Some(Ok(serial));
        }
        if let Ok(model) = buffer.model() {
            res.model = Some(Ok(model));
        }
        if let Ok(firmware) = buffer.firmware() {
            res.firmware = Some(Ok(firmware));
        }

        res
    }
}

impl Health for Device {
    fn health(&self, _args: &HealthArgs) -> HealthResult {
        let mut res: HealthResult = Default::default();

        let dev = self.i.make_device();

        let Ok(buffer) = dev.get_smart_log() else {
            return res;
        };

        if let Ok(data_units_written) = buffer.data_units_written() {
            res.data_units_written = Some(Ok(data_units_written));
        }
        if let Ok(data_units_read) = buffer.data_units_read() {
            res.data_units_read = Some(Ok(data_units_read));
        }
        if let Ok(percentage_used) = buffer.percentage_used() {
            res.percentage_used = Some(Ok(percentage_used));
        }

        let mut cb: Vec<CriticalBits> = Vec::new();
        if let Ok(spare_below_threshold) = buffer.cw_spare_below_threshold() {
            if spare_below_threshold {
                cb.push(CriticalBits::SpareBelowThreshold);
            }
        }
        if let Ok(temperature) = buffer.cw_temperature() {
            if temperature {
                cb.push(CriticalBits::Temperature);
            }
        }
        if let Ok(subsystem_degraded) = buffer.cw_subsystem_degraded() {
            if subsystem_degraded {
                cb.push(CriticalBits::SubsystemDegraded);
            }
        }
        if let Ok(read_only_mode) = buffer.cw_read_only() {
            if read_only_mode {
                cb.push(CriticalBits::ReadOnlyMode);
            }
        }
        if let Ok(volatile_backup_failed) = buffer.cw_backup() {
            if volatile_backup_failed {
                cb.push(CriticalBits::VolatileBackupFailed);
            }
        }
        if let Ok(persistent_memory_region_unavailable) = buffer.cw_persistent_memory_region() {
            if persistent_memory_region_unavailable {
                cb.push(CriticalBits::PersistentMemoryRegionUnreliable);
            }
        }
        res.critical_bits = Some(Ok(cb));

        // TODO: Implement update_available, disposition, tasks.
        res.disposition = Some(Ok(HealthDisposition::Excellent));
        res.tasks = Some(Ok(HousekeepingTasks::None));

        res
    }
}
