<template>

</template>

<script lang="ts" setup>
import { watchEffect } from 'vue';
import useResource from '@/compositions/use-resource';

const props = defineProps<{
    id: number
}>();
console.log(props);
const { loading, resource } = useResource(props.id);
watchEffect(() => {
    console.log(resource.value)
});
</script>

<style lang="scss" scoped>
</style>
