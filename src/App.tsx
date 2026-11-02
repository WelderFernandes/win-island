import { useState, useEffect } from "react";
import { motion, AnimatePresence, type Transition } from "framer-motion";
import { Play, Pause, SkipForward, SkipBack, Music, Volume2 } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface MediaSessionInfo {
  title: String;
  artist: String;
  is_playing: boolean;
  has_media: boolean;
}

export default function App() {
  const [isExpanded, setIsExpanded] = useState(false);
  const [mediaInfo, setMediaInfo] = useState<MediaSessionInfo>({
    title: "Nenhuma mídia",
    artist: "Windows Media",
    is_playing: false,
    has_media: false,
  });

  // Polling GSMTC system media state every second
  useEffect(() => {
    const fetchMedia = async () => {
      try {
        const info = await invoke<MediaSessionInfo>("get_media_info");
        setMediaInfo(info);
      } catch (err) {
        console.error("Erro ao obter mídia do Windows:", err);
      }
    };

    fetchMedia();
    const interval = setInterval(fetchMedia, 1000);
    return () => clearInterval(interval);
  }, []);

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

  // macOS elastic spring physics
  const springTransition: Transition = {
    type: "spring",
    stiffness: 380,
    damping: 26,
    mass: 0.8,
  };

  return (
    <div className="island-container">
      <motion.div
        className="island-pill"
        onClick={() => setIsExpanded(!isExpanded)}
        layout
        initial={false}
        animate={{
          width: isExpanded ? 380 : 210,
          height: isExpanded ? 160 : 38,
          borderRadius: isExpanded ? 32 : 19,
        }}
        transition={springTransition}
      >
        <AnimatePresence mode="wait">
          {!isExpanded ? (
            <motion.div
              key="compact"
              className="island-compact"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.15 }}
            >
              <div className="left-icon">
                <Music size={15} />
                <span style={{ maxWidth: 120, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
                  {mediaInfo.has_media ? mediaInfo.title : "Dynamic Island"}
                </span>
              </div>
              <div className="right-visualizer">
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
          ) : (
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
                <div className="album-cover">
                  <Music size={24} color="#ffffff" />
                </div>
                <div className="track-details">
                  <span className="track-title">{mediaInfo.title}</span>
                  <span className="track-artist">{mediaInfo.artist}</span>
                </div>
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
                  onClick={() => setIsExpanded(false)}
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
