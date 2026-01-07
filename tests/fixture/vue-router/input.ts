// Vue Router 测试
const router = useRouter();
const route = useRoute();

const navigate = () => {
  router.push('/home');
};

const currentPath = computed(() => route.path);

