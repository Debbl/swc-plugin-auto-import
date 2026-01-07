import { useCallback, useEffect, useState } from "react";
function Counter() {
    const [count, setCount] = useState(0);
    const [user, setUser] = useState(null);
    useEffect(()=>{
        console.log('Count changed:', count);
    }, [
        count
    ]);
    const increment = useCallback(()=>{
        setCount((c)=>c + 1);
    }, []);
    return <div onClick={increment}>{count}</div>;
}

