import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
const router = useRouter();
const route = useRoute();
const navigate = ()=>{
    router.push('/home');
};
const currentPath = computed(()=>route.path);

