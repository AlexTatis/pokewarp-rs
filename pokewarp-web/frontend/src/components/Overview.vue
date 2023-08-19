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
    <div v-if="pkm && pkm.pkm_id" class="bg-white shadow-md rounded-s-md md:overflow-hidden">
        <div class="bg-slate-700 py-3 px-3 mb-8 text-white flex items-center justify-between sticky top-0 z-50">
            <img :src="`/sprites/sprites/items/${pkm.pokeball.sprite}.png`" class="pixelated w-14">
            <p class="font-bold text-2xl ">{{ pkm.nickname }}</p>
            <p class="text-xl">Lvl. {{ pkm.level }}</p>
        </div>
        <div class="flex h-48 mb-10 px-3 md:px-0">
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
        <div v-if="activeTab == 'general'" class=" ml-3">
            <div class="grid grid-cols-2 gap-3 mb-6 w-fit">
                <p class="text-white bg-slate-700 font-semibold py-1 px-4 text-center">Dex Id.</p>
                <p class="p-1">{{ pkm.pkm_id }}</p>

                <p class="text-white bg-slate-700 font-semibold py-1 px-4 text-center">Species</p>
                <p class="p-1">{{ pkm.species }}</p>

                <p class="text-white bg-slate-700 font-semibold py-1 px-4 text-center">Held Item</p>
                <p class="p-1">{{ pkm.item_id ? pkm.item.name : "-" }}</p>

                <p class="text-white bg-slate-700 font-semibold py-1 px-4 text-center">Gender</p>
                <p class="p-1">{{ pkm.gender }}</p>

            </div>
            <div class="rounded bg-base-300 overflow-hidden border-black md:w-96 shadow-xl mx-auto w-80">
                <div class="absolute z-50 text-white w-max">
                    <p class="p-2 font-bold">ORIGINAL TRAINER CARD</p>
                    <div class="flex justify-between items-center gap-2 w-80 md:w-96">
                        <div>
                            <p class="pl-2 py-1 font-semibold">NAME: {{ pkm.ot_name.toUpperCase() }}</p>
                            <p class="pl-2 py-1 font-semibold">ID: {{ pkm.ot_id }}</p>
                            <p class="pl-2 py-1 font-semibold">SECRET ID: {{ pkm.ot_sid }}</p>
                        </div>
                        <img src="https://play.pokemonshowdown.com/sprites/trainers/hilbert.png" class="h-36 pixelated">
                    </div>
                </div>
                <img src="/trainer-card.svg" alt="" class="relative">
            </div>

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