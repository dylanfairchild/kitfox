import { ref, computed } from "vue"
import { defineStore, acceptHMRUpdate } from "pinia"

function cleanIdentify(identify) {
    let cleaned = {}
    cleaned.model = (identify.model && identify.model.Ok) ? identify.model.Ok : "Model Not Found"
    cleaned.serial = (identify.serial && identify.serial.Ok) ? identify.serial.Ok : "Serial Not Found"
    cleaned.firmware = (identify.firmware && identify.firmware.Ok) ? identify.firmware.Ok : "Firmware Not Found"
    cleaned.name = (identify.name && identify.name.Ok) ? identify.name.Ok : ""
    return cleaned
}

function cleanHealth(health) {
    let cleaned = {}
    cleaned.disposition = (health.disposition && health.disposition.Ok) ? health.disposition.Ok : "N/A"
    cleaned.tasks = (health.tasks && health.tasks.Ok) ? health.tasks.Ok : "N/A"
    cleaned.data_units_written = (health.data_units_written && health.data_units_written.Ok) ? health.data_units_written.Ok : "N/A"
    cleaned.data_units_read = (health.data_units_read && health.data_units_read.Ok) ? health.data_units_read.Ok : "N/A"
    cleaned.percentage_used = (health.percentage_used && health.percentage_used.Ok) ? health.percentage_used.Ok : "N/A"
    cleaned.critical_bits = (health.critical_bits && health.critical_bits.Ok) ? health.critical_bits.Ok : []
    cleaned.update_available = (health.update_available && health.update_available.Ok) ? health.update_available.Ok : "N/A"
    return cleaned
}

export const useResourceStore = defineStore('resource', () => {
    const selectedIdentifier = ref("")
    const allIdentifier = ref([])
    const allIdentify = ref({})
    const allHealth = ref({})

    const specificIdentify = computed(() => {
        return (identifier) => {
            const all = allIdentify.value
            if (all.payload) {
                return cleanIdentify(all.payload.find(pl => pl.identifier && pl.identifier == identifier).payload.find(pl => pl.type == "IdentifyResult"))
            }
            return {}
        }
    })

    const selectedIdentify = computed(() => {
        const all = allIdentify.value
        const selected = selectedIdentifier.value
        if (all.payload) {
            return cleanIdentify(all.payload.find(pl => pl.identifier && pl.identifier == selected).payload.find(pl => pl.type == "IdentifyResult"))
        }
        return {}
    })

    const selectedHealth = computed(() => {
        const all = allHealth.value
        const selected = selectedIdentifier.value
        if (all.payload) {
            return cleanHealth(all.payload.find(pl => pl.identifier && pl.identifier == selected).payload.find(pl => pl.type == "HealthResult"))
        }
        return {}
    })

    return {
        selectedIdentifier,
        selectedIdentify,
        specificIdentify,
        allIdentifier,
        allIdentify,
        selectedHealth,
        allHealth,
    }
})

// TODO: I added this like it suggests https://pinia.vuejs.org/cookbook/hot-module-replacement.html but it 
//       does not seem that it actually reloads changes I make to computed methods. Something else is missing
//       in getting that to work.
//
//       If I do put this in here and the code below is uncommented I noticed that I will start having nothing
//       in my state on hot-module reload.
// if (import.meta.hot) {
//     import.meta.hot.accept(acceptHMRUpdate(useResourceStore, import.meta.hot))
// }