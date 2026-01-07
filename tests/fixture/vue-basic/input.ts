// Vue 基本用法测试
const count = ref(0);
const doubled = computed(() => count.value * 2);

watch(count, (newVal) => {
  console.log(newVal);
});

onMounted(() => {
  console.log('mounted');
});

