<script setup lang="ts">
import BoxSelector from './BoxSelector.vue'
import Box from './Box.vue'
import { currentBox, emitter, EMPTY_PK5 } from '../utils';
import { ref } from 'vue';

const maxBoxes = 12
const boxes = ref(Array(maxBoxes).fill(undefined).map(() => Array(30).fill(EMPTY_PK5)))

emitter.on("boxChange", boxChange => {
    boxes.value[boxChange.box][boxChange.slot] = boxChange.pkm
})

</script>

<template>
    <BoxSelector :max-boxes="maxBoxes" v-model:current-box="currentBox" />
    <template v-for="i in boxes.length">
        <Box v-model:boxes="boxes[i - 1]" v-if="i - 1 == currentBox" :key="i" :index="i - 1" />
    </template>
</template>