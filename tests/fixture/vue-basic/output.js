import { computed, onMounted, ref, watch } from "vue";
const count = ref(0);
const doubled = computed(()=>count.value * 2);
watch(count, (newVal)=>{
    console.log(newVal);
});
onMounted(()=>{
    console.log('mounted');
});

