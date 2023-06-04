import "../css/moveLog.css";

const MoveLog = ({moveLog}: {moveLog: Array<string>}) => {
    return (
        <div className="move-log-container">
            <h1 className="move-log-title">Move Log</h1>
            <div className="move-log">
                { moveLog.map((move, i) => {
                    return <p key={i}>{i} - {move}</p>
                })}
            </div>
        </div>
    )
}

export default MoveLog;