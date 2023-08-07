<script setup lang="ts">
import { ref, watch } from 'vue';
import { PK5, emitter } from '../utils'

const PROPS = defineProps<{
    pkm: PK5
}>()

const selected = ref(false);

watch(selected, (watched: boolean) => {
    if (!PROPS.pkm) return;


    if (watched) {
        emitter.emit('setOverview', PROPS.pkm)
        emitter.emit('newSlotSelected')
        emitter.on('newSlotSelected', newSlotSelectedHandler)
        return;
    }

    emitter.off('newSlotSelected', newSlotSelectedHandler)

})

function newSlotSelectedHandler() {
    selected.value = false
}

</script>
<template>
    <div class="md:w-96 md:h-32 h-24 w-24 shadow-lg rounded-full bg-white"
        :class="{ ' border-red-500 border-dashed border-2': selected }" @click="selected = !selected" onclick="if (window.innerWidth <= 640) my_modal_5.showModal()">
        <div v-if="PROPS.pkm.id != 0" class="flex flex-row items-center h-full pixelated gap-2 pl-2">
            <img :src="`/pokesprite/pokemon-gen8/regular/${PROPS.pkm.species.toLowerCase()}.png`" alt=""
                class="w-28 animate-bounce">
            <div class="md:block hidden">
                <p class="text-3xl font-bold">{{ PROPS.pkm.nickname }}</p>
                <p class="font-semibold">Lvl. {{ PROPS.pkm.level }}</p>
            </div>
            <div class="tooltip m-auto md:block hidden" :data-tip="pkm.item.name" v-if="pkm.item.num">
                <img :src="`/sprites/sprites/items/${PROPS.pkm.item.sprite}.png`" class="pixelated w-14">
            </div>
        </div>
    </div>
</template>

<style>
.pixelated {
    image-rendering: pixelated;
}
</style>