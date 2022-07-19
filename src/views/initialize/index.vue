<template>
    <div class="progress">
        <NP>{{ message }}</NP>
        <NProgress v-if="step === 'downloading'" :percentage="percentage" :processing="percentage < 100"
            :show-indicator="false" />
        <NProgress v-else :percentage="importPercentage" :processing="importPercentage < 100" :show-indicator="false" />
    </div>
</template>

<script lang="ts" setup>
import { NP, NProgress } from 'naive-ui';
import useInitialize from '@/compositions/use-initialize';

const [percentage, importPercentage, message, step] = useInitialize();

</script>

<style lang="scss" scoped>
.progress {
    box-sizing: border-box;
    padding: 20px;
    font-size: .9em;
    text-align: center;
    width: 100%;
    position: absolute;
    top: 40%;
    left: 0;
    transform: translateY(-50%);
}
</style>
