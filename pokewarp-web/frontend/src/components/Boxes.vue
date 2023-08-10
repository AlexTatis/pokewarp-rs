<script setup lang="ts">
    import BoxSelector from './BoxSelector.vue'
    import Box from './Box.vue'
    import { maxBoxes, boxes, currentBox } from '../utils';

</script>

<template>
    <BoxSelector :max-boxes="maxBoxes" v-model:current-box="currentBox" />
    <template v-for="i in boxes.length">
        <Box v-model:boxes="boxes[i - 1]" v-if="i - 1 == currentBox" :key="i" :index="i - 1" />
    </template>
</template>