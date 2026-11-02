import { useState, useEffect } from "react";
import { motion, AnimatePresence, type Transition } from "framer-motion";
import { Play, Pause, SkipForward, SkipBack, Music, Volume2, Battery, BatteryCharging } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface MediaSessionInfo {
  title: string;
  artist: string;
  is_playing: boolean;
  has_media: boolean;
  thumbnail_base64?: string | null;
}

interface SystemStatusInfo {
  battery_percent: number;
  is_charging: boolean;
  has_battery: boolean;
}

// 3 Estágios da Ilha:
// 1. "bubble": escondidinha no topo como uma bolha discreta
// 2. "compact": ilha em si com informações ao passar o mouse
// 3. "expanded": painel completo aberto ao clicar
type IslandStage = "bubble" | "compact" | "expanded";

export default function App() {
  const [stage, setStage] = useState<IslandStage>("bubble");
  const [mediaInfo, setMediaInfo] = useState<MediaSessionInfo>({
    title: "Nenhuma mídia",
    artist: "Windows Media",
    is_playing: false,
    has_media: false,
    thumbnail_base64: null,
  });
  const [systemStatus, setSystemStatus] = useState<SystemStatusInfo>({
    battery_percent: 100,
    is_charging: true,
    has_battery: false,
  });

  // Polling GSMTC system media e bateria
  useEffect(() => {
    const fetchStatus = async () => {
      try {
        const media = await invoke<MediaSessionInfo>("get_media_info");
        setMediaInfo(media);

        const sys = await invoke<SystemStatusInfo>("get_system_status");
        setSystemStatus(sys);
      } catch (err) {
        console.error("Erro ao obter status do Windows:", err);
      }
    };

    fetchStatus();
    const interval = setInterval(fetchStatus, 1000);
    return () => clearInterval(interval);
  }, []);

  const handleMouseEnter = () => {
    if (stage === "bubble") {
      setStage("compact");
    }
  };

  const handleMouseLeave = () => {
    if (stage === "compact") {
      setStage("bubble");
    }
  };

  const handlePillClick = () => {
    if (stage === "bubble" || stage === "compact") {
      setStage("expanded");
    }
  };

  const handleClose = (e: React.MouseEvent) => {
    e.stopPropagation();
    setStage("bubble");
  };

  const handleTogglePlay = async (e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await invoke("toggle_play_pause");
      setMediaInfo((prev) => ({ ...prev, is_playing: !prev.is_playing }));
    } catch (err) {
      console.error(err);
    }
  };

  const handleSkipNext = async (e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await invoke("skip_next");
    } catch (err) {
      console.error(err);
    }
  };

  const handleSkipPrev = async (e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await invoke("skip_previous");
    } catch (err) {
      console.error(err);
    }
  };

  // Física elástica de mola estilo macOS
  const springTransition: Transition = {
    type: "spring",
    stiffness: 420,
    damping: 28,
    mass: 0.75,
  };

  return (
    <div className="island-container">
      <motion.div
        className="island-pill"
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        onClick={handlePillClick}
        layout
        initial={false}
        animate={{
          width: stage === "bubble" ? 54 : stage === "compact" ? 230 : 380,
          height: stage === "bubble" ? 14 : stage === "compact" ? 38 : 165,
          borderBottomLeftRadius: stage === "bubble" ? 12 : stage === "compact" ? 20 : 32,
          borderBottomRightRadius: stage === "bubble" ? 12 : stage === "compact" ? 20 : 32,
          borderTopLeftRadius: 0,
          borderTopRightRadius: 0,
        }}
        transition={springTransition}
      >
        <AnimatePresence mode="wait">
          {stage === "bubble" && (
            <motion.div
              key="bubble"
              className="island-bubble"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.15 }}
            >
              <div className="bubble-glow" />
            </motion.div>
          )}

          {stage === "compact" && (
            <motion.div
              key="compact"
              className="island-compact"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.15 }}
            >
              <div className="left-icon">
                {mediaInfo.thumbnail_base64 ? (
                  <img
                    src={mediaInfo.thumbnail_base64}
                    alt="Cover"
                    style={{ width: 18, height: 18, borderRadius: 5, objectFit: "cover" }}
                  />
                ) : (
                  <Music size={15} />
                )}
                <span style={{ maxWidth: 110, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
                  {mediaInfo.has_media ? mediaInfo.title : "Dynamic Island"}
                </span>
              </div>
              <div className="right-visualizer">
                {systemStatus.has_battery && (
                  <div style={{ display: "flex", alignItems: "center", gap: 3, marginRight: 6, fontSize: 11, opacity: 0.8 }}>
                    {systemStatus.is_charging ? <BatteryCharging size={13} color="#22c55e" /> : <Battery size={13} />}
                    <span>{systemStatus.battery_percent}%</span>
                  </div>
                )}
                {[0.4, 0.8, 0.5, 0.9, 0.3].map((height, i) => (
                  <motion.div
                    key={i}
                    className="bar"
                    animate={
                      mediaInfo.is_playing
                        ? {
                            height: [height * 14, 16, 4, height * 12],
                          }
                        : { height: 4 }
                    }
                    transition={{
                      repeat: Infinity,
                      repeatType: "reverse",
                      duration: 0.5 + i * 0.1,
                      ease: "easeInOut",
                    }}
                  />
                ))}
              </div>
            </motion.div>
          )}

          {stage === "expanded" && (
            <motion.div
              key="expanded"
              className="island-expanded"
              initial={{ opacity: 0, y: 4 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -4 }}
              transition={{ duration: 0.2 }}
              onClick={(e) => e.stopPropagation()}
            >
              <div className="expanded-header">
                <div className="album-cover" style={{ overflow: "hidden" }}>
                  {mediaInfo.thumbnail_base64 ? (
                    <img
                      src={mediaInfo.thumbnail_base64}
                      alt="Album Cover"
                      style={{ width: "100%", height: "100%", objectFit: "cover" }}
                    />
                  ) : (
                    <Music size={24} color="#ffffff" />
                  )}
                </div>
                <div className="track-details">
                  <span className="track-title">{mediaInfo.title}</span>
                  <span className="track-artist">{mediaInfo.artist}</span>
                </div>
                {systemStatus.has_battery && (
                  <div style={{ marginLeft: "auto", display: "flex", alignItems: "center", gap: 4, fontSize: 12, opacity: 0.8 }}>
                    {systemStatus.is_charging ? <BatteryCharging size={15} color="#22c55e" /> : <Battery size={15} />}
                    <span>{systemStatus.battery_percent}%</span>
                  </div>
                )}
              </div>

              <div className="progress-container">
                <div className="progress-bar" />
              </div>

              <div className="expanded-controls">
                <Volume2 size={18} color="rgba(255, 255, 255, 0.6)" />

                <div className="playback-buttons">
                  <button className="ctrl-btn" onClick={handleSkipPrev} aria-label="Anterior">
                    <SkipBack size={18} />
                  </button>

                  <button
                    className="ctrl-btn"
                    onClick={handleTogglePlay}
                    aria-label={mediaInfo.is_playing ? "Pausar" : "Reproduzir"}
                  >
                    {mediaInfo.is_playing ? <Pause size={22} fill="white" /> : <Play size={22} fill="white" />}
                  </button>

                  <button className="ctrl-btn" onClick={handleSkipNext} aria-label="Próximo">
                    <SkipForward size={18} />
                  </button>
                </div>

                <button
                  className="ctrl-btn"
                  onClick={handleClose}
                  style={{ fontSize: 12, opacity: 0.6 }}
                >
                  Fechar
                </button>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
      </motion.div>
    </div>
  );
}
