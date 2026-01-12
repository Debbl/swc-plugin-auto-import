const { x, y } = useMouse()
const data = useMyFetch('/api/users')
const response = axios.get('/api/data')
const result = _.map([1, 2, 3], (n) => n * 2)
