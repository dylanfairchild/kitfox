<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";

import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { useResourceStore } from './stores/resource'

const store = useResourceStore()

interface RequestPayload {
    payload: any[]; //TODO: Define a type for my payloads so I don't need this any[].
}
async function request(payload: RequestPayload) {
  return JSON.parse(await invoke("request", { requestPayload: JSON.stringify(payload)}))
}

async function scan() {
  store.allIdentifier = JSON.parse(await invoke("scan"))
}

async function identify(identifiers: Array<string>) {
  let requestPayload: RequestPayload = {
    payload: []
  }
  identifiers.forEach(e => {
    requestPayload.payload.push(
      {
        identifier: e,
        payload: [{
          type: "IdentifyArgs"
        }]
      }
    )
  })

  store.allIdentify = await request(requestPayload)
}

async function health(identifiers: Array<string>) {
  let requestPayload: RequestPayload = {
    payload: []
  }
  identifiers.forEach(e => {
    requestPayload.payload.push(
      {
        identifier: e,
        payload: [{
          type: "HealthArgs"
        }]
      }
    )
  })

  store.allHealth = await request(requestPayload)
}

onMounted(() => {
  scan().then(() => {
    identify(store.allIdentifier)
    health(store.allIdentifier)
  })
})

</script>

<template>
  <v-app>
    <v-navigation-drawer permanent>
      <v-list class="pt-2 pl-0 pb-2 pr-0 ma-0">
        <v-list-subheader>Storage</v-list-subheader>
        <!-- v-list-item v-for="identifier in identifiers" :active="false" :value="identifier" class="pl-0" :to="{ name: 'storage', params: { id: identifier } }" -->
        <v-list-item v-for="identifier in store.allIdentifier" :active="false" :value="identifier" @click="store.selectedIdentifier = identifier" class="pl-0" :to="{ name: 'storage', params: { id: identifier } }">
          <v-row justify="start" align="center" no-gutters>
            <v-col cols="3"><v-img height="50" src="/images/ssd.svg"></v-img></v-col>
            <v-col cols="9">
              <div class="text-subtitle-2 text-no-wrap text-truncate">{{ store.specificIdentify(identifier).model }}</div>
              <div class="text-caption">{{ store.specificIdentify(identifier).name }}</div>
            </v-col>
          </v-row>
        </v-list-item>
        <v-list-subheader>Compute</v-list-subheader>
        <v-list-subheader>Network</v-list-subheader>
        <v-list-subheader>Peripherals & HID</v-list-subheader>
      </v-list>

      <template v-slot:append>
        <div class="pa-4">
          <v-btn :to="{ name: 'settings' }">Kitfox Configuration</v-btn>
        </div>
      </template>

    </v-navigation-drawer>
    <v-main>
      <router-view></router-view>
      <!-- <v-container>
      <v-row justify="start">
      <v-col
        v-for="identifier in identifiers"
        cols="6"
      >
        <v-card height="150">
          <v-row justify="start">
            <v-col cols="5">
              <v-img height="150" cover src="https://cdn.vuetifyjs.com/images/parallax/material.jpg"></v-img>
            </v-col>
            <v-col cols="7" class="pt-8 pb-8">
              <h3>Samsung SSD 980 PRO 1TB</h3>
              <v-row no-gutters>
                  <v-col cols="4">Serial: </v-col>
                  <v-col cols="8">S5P2NS0RB28516B</v-col>
              </v-row>
              <v-row no-gutters>
                <v-col cols="4">Firmware: </v-col>
                <v-col cols="8">3B2QGXA7</v-col>
              </v-row>
            </v-col>
          </v-row>
        </v-card>
      </v-col>
    </v-row>
      </v-container> -->
    </v-main>
  </v-app>
</template>


<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
