import { ref, computed } from 'vue';
const count = ref(0);
const doubled = computed(()=>count.value * 2);

