<script setup lang="ts">
    import BoxSelector from './BoxSelector.vue'
    import Box from './Box.vue'
    import { ref } from 'vue';
    import { EMPTY_PK5 } from '../utils';

    const maxBoxes = 12
    const currentBox = ref(0)
    const boxes = ref(Array(maxBoxes).fill(ref(Array(30).fill(EMPTY_PK5))))

</script>

<template>
    <BoxSelector :max-boxes="maxBoxes" v-model:current-box="currentBox" />
    <template v-for="i in boxes.length">
        <Box v-model:boxes="boxes[i - 1]" v-if="i - 1 == currentBox" :key="i" />
    </template>
</template>