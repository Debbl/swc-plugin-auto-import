import { useEffect, useRef, useState } from "react";
function App() {
    const [count, setCount] = useState(0);
    const ref = useRef(null);
    useEffect(()=>{
        console.log("Effect");
    }, []);
    return <div ref={ref}>{count}</div>;
}
