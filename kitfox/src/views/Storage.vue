<script setup lang="ts">
import { useResourceStore } from '../stores/resource'

const store = useResourceStore()

</script>

<template>
    <v-container>
        <v-row>
            <v-col cols="12">
                <v-card>
                    <v-card-title>Identification</v-card-title>
                    <v-card-text>
                        <table>
                            <tbody>
                                <tr>
                                    <td class="data-label">Model</td>
                                    <td>{{ store.selectedIdentify.model }}</td>
                                </tr>
                                <tr>
                                    <td class="data-label">Serial</td>
                                    <td>{{ store.selectedIdentify.serial }}</td>
                                </tr>
                                <tr>
                                    <td class="data-label">Firmware</td>
                                    <td>{{  store.selectedIdentify.firmware }}</td>
                                </tr>
                                <tr>
                                    <td class="data-label">Name</td>
                                    <td>{{ store.selectedIdentify.name }}</td>
                                </tr>
                            </tbody>
                        </table>
                    </v-card-text>
                </v-card>
            </v-col>
            <v-col cols="12">
                <v-card>
                    <v-card-title>Health & Housekeeping</v-card-title>
                    <v-card-text>
                        <v-row>
                            <v-col>
                                <div class="text-subtitle-1">Notices</div>
                                <tr>
                                    <td class="data-label">General Health</td>
                                    <td>{{ store.selectedHealth.disposition }}</td>
                                </tr>
                                <tr>
                                    <td class="data-label">Housekeeping Tasks</td>
                                    <td>{{ store.selectedHealth.tasks }}</td>
                                </tr>
                            </v-col>
                        </v-row>
                        <v-row>
                            <v-col>
                                <div class="text-subtitle-1">Summary</div>
                                <tr>
                                    <td class="data-label">Data Units Written</td>
                                    <td>{{ store.selectedHealth.data_units_written }}</td>
                                </tr>
                                <tr>
                                    <td class="data-label">Data Units Read</td>
                                    <td>{{ store.selectedHealth.data_units_read }}</td>
                                </tr>
                                <tr>
                                    <td class="data-label">Percentage Used</td>
                                    <td>{{ store.selectedHealth.percentage_used }}%</td>
                                </tr>
                                <tr>
                                    <td class="data-label">Critical Bits</td>
                                    <td>{{ store.selectedHealth.critical_bits }}<br/>None (DEBUG)<br/>And<br/>Some</td>
                                </tr>
                                <tr>
                                    <td class="data-label">Updates</td>
                                    <td>{{ store.selectedHealth.update_available }}</td>
                                </tr>
                            </v-col>
                            <v-col>
                                <div class="text-subtitle-1">Additional Controls</div>
                                <v-btn>View Full SMART Data</v-btn>
                            </v-col>
                        </v-row>
                    </v-card-text>
                </v-card>
            </v-col>
            <v-col cols="12">
                <v-card>
                    <v-card-title>Performance</v-card-title>
                    <v-card-text>
                        <v-row>
                            <v-col>
                                <div class="text-subtitle-1">Benchmark</div>
                                <div style="background-color: grey; width: 650px; height:150px;"></div>
                                <v-row>
                                    <v-col>
                                        <div class="text-subtitle-2">Settings</div>
                                        <div class="ml-2">
                                            <v-combobox
                                                label="Type"
                                                :items="['Random Read', 'Random Write', 'Sequential Read', 'Sequential Write']"
                                                variant="solo">
                                            </v-combobox>
                                            <v-combobox
                                                label="Queue Depth"
                                                :items="['1', '4', '8', '16']"
                                                variant="solo">
                                            </v-combobox>
                                        </div>
                                    </v-col>
                                    <v-col>
                                        <v-btn class="mt-2">Start Benchmark</v-btn>
                                    </v-col>
                                </v-row>
                            </v-col>
                        </v-row>
                    </v-card-text>
                </v-card>
            </v-col>
        </v-row>
    </v-container>
    <v-card></v-card>


    <div>Storage Page {{ $route.params.id }} -- {{  store.selectedIdentifier }} -- {{ JSON.stringify(store.selectedIdentify) }} -- {{  JSON.stringify(store.allIdentify) }}</div>
<div> Info (Model, Serial, Firmware, etc.)</div>
<div> Health / SMART / Wear Leveling </div>
<div> Performance (Benchmark?) </div>
<div> Capacity / Fill </div>
</template>

<style>

.data-label {
    text-align: right;
    padding-right: 10px;
    font-weight: 700;
}

</style>