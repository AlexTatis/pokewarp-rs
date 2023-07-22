<script setup lang="ts">
import { ref } from 'vue';
import { PK5, emitter } from '../utils';
import { Radar } from 'vue-chartjs'
import { Chart, Title, Tooltip, Legend, RadialLinearScale, PointElement, LineElement } from 'chart.js'

const pkm = ref<PK5>()
const activeTab = ref<"general" | "ivs-evs">("general")

emitter.on('setOverview', receivedPkm => pkm.value = receivedPkm)

Chart.register(Title, Tooltip, Legend, RadialLinearScale, PointElement, LineElement)

</script>

<template>
    <div v-if="pkm && pkm.id" class="bg-white shadow-md rounded-s-md overflow-hidden">
        <div class="bg-slate-700 py-3 px-3 mb-8 text-white flex items-center justify-between">
            <img :src="`/sprites/sprites/items/${pkm.pokeball.sprite}.png`" class="pixelated w-14">
            <p class="font-bold text-2xl ">{{ pkm.nickname }}</p>
            <p class="text-xl">Lvl. {{ pkm.level }}</p>
        </div>
        <div class="flex h-48 mb-4">
            <div class="ml-3 pr-3">
                <img :src="`https://projectpokemon.org/images/normal-sprite/${pkm.species.toLowerCase()}.gif`" alt=""
                    class="pixelated w-48">
            </div>
            <div>
                <div v-for="move in pkm.moves" class="-skew-x-12 bg-slate-700">
                    <p class="text-white px-3 mb-2 py-1 font-semibold skew-x-12 flex items-center gap-2"><img
                            :src="`/type-icons/${move.type.toLowerCase()}.png`" class="w-8"> {{ move.name }}</p>
                </div>
            </div>
        </div>
        <div class="tabs tabs-boxed w-fit mx-auto mb-4">
            <a class="tab" :class="{ 'tab-active': activeTab == 'general' }"
                @click="() => activeTab = 'general'">General</a>
            <a class="tab" :class="{ 'tab-active': activeTab == 'ivs-evs' }" @click="() => activeTab = 'ivs-evs'">IVs /
                EVs</a>
        </div>
        <div v-if="activeTab == 'general'" class="grid grid-cols-2 ml-3 w-fit gap-3">
            <p class="text-white bg-slate-700 font-semibold py-1 px-4 text-center">Dex Id.</p>
            <p class="p-1">{{ pkm.id }}</p>

            <p class="text-white bg-slate-700 font-semibold py-1 px-4 text-center">Species</p>
            <p class="p-1">{{ pkm.species }}</p>

            <p class="text-white bg-slate-700 font-semibold py-1 px-4 text-center">Held Item</p>
            <p class="p-1">{{ pkm.item_id ? pkm.item.name : "-" }}</p>

            <p class="text-white bg-slate-700 font-semibold py-1 px-4 text-center">Gender</p>
            <p class="p-1">{{ pkm.gender }}</p>

        </div>
        <Radar v-if="activeTab == 'ivs-evs'" :options="{ responsive: true }" :data="{
            labels: ['Atk', 'Def', 'HP', 'SpAtk', 'SpDef', 'Speed'],
            datasets: [
                {
                    label: 'IVs',
                    data: Object.values(pkm.ivs),
                    backgroundColor: [
                        'rgba(255, 99, 132, 0.2)',
                        'rgba(54, 162, 235, 0.2)',
                        'rgba(255, 206, 86, 0.2)',
                        'rgba(75, 192, 192, 0.2)',
                        'rgba(153, 102, 255, 0.2)',
                        'rgba(255, 159, 64, 0.2)'
                    ],
                    borderColor: [
                        'rgba(255, 99, 132, 1)',
                        'rgba(54, 162, 235, 1)',
                        'rgba(255, 206, 86, 1)',
                        'rgba(75, 192, 192, 1)',
                        'rgba(153, 102, 255, 1)',
                        'rgba(255, 159, 64, 1)'
                    ]
                },
                {
                    label: 'EVs',
                    data: Object.values(pkm.evs),
                    backgroundColor: [
                        'rgba(255, 99, 132, 0.2)',
                        'rgba(54, 162, 235, 0.2)',
                        'rgba(255, 206, 86, 0.2)',
                        'rgba(75, 192, 192, 0.2)',
                        'rgba(153, 102, 255, 0.2)',
                        'rgba(255, 159, 64, 0.2)'
                    ],
                    borderColor: [
                        'rgba(255, 99, 132, 1)',
                        'rgba(54, 162, 235, 1)',
                        'rgba(255, 206, 86, 1)',
                        'rgba(75, 192, 192, 1)',
                        'rgba(153, 102, 255, 1)',
                        'rgba(255, 159, 64, 1)'
                    ]
                },
            ]
        }" />
    </div>
</template>