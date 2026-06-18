import { useState, useCallback } from 'react';
import {
  TrackType,
  type TimelineTrack,
  type TimelineElement,
  type TimelineProject,
  DEFAULT_TRACK_HEIGHTS,
  TRACK_COLORS,
  TRACK_TYPE_LABELS,
} from '../../types/timeline';
import { Lock, Pause, Play, Plus, Unlock, Volume2, VolumeX } from 'lucide-react';

const DEFAULT_TRACKS: TimelineTrack[] = [
  {
    id: 'video_track_1',
    name: '视频轨道 1',
    trackType: TrackType.Video,
    elements: [],
    muted: false,
    locked: false,
    height: DEFAULT_TRACK_HEIGHTS[TrackType.Video],
  },
  {
    id: 'audio_track_1',
    name: '音频轨道 1',
    trackType: TrackType.Audio,
    elements: [],
    muted: false,
    locked: false,
    height: DEFAULT_TRACK_HEIGHTS[TrackType.Audio],
  },
  {
    id: 'document_track_1',
    name: '文档轨道 1',
    trackType: TrackType.Document,
    elements: [],
    muted: false,
    locked: false,
    height: DEFAULT_TRACK_HEIGHTS[TrackType.Document],
  },
  {
    id: 'subtitle_track_1',
    name: '字幕轨道 1',
    trackType: TrackType.Subtitle,
    elements: [],
    muted: false,
    locked: false,
    height: DEFAULT_TRACK_HEIGHTS[TrackType.Subtitle],
  },
];

export function ProjectTimeline() {
  const [project, setProject] = useState<TimelineProject>({
    name: '未命名项目',
    totalDuration: 300,
    tracks: DEFAULT_TRACKS,
    currentTime: 0,
    zoomLevel: 20,
    isPlaying: false,
  });

  const [selectedTrackId, setSelectedTrackId] = useState<string | null>(null);
  const [selectedElementId, setSelectedElementId] = useState<string | null>(null);

  const handlePlayPause = useCallback(() => {
    setProject((prev) => ({ ...prev, isPlaying: !prev.isPlaying }));
  }, []);

  const handleZoomChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    setProject((prev) => ({ ...prev, zoomLevel: parseInt(e.target.value) }));
  }, []);

  const handleAddTrack = useCallback((trackType: TrackType) => {
    const newTrack: TimelineTrack = {
      id: `${trackType.toLowerCase()}_track_${Date.now()}`,
      name: `${TRACK_TYPE_LABELS[trackType]} ${project.tracks.filter((t) => t.trackType === trackType).length + 1}`,
      trackType,
      elements: [],
      muted: false,
      locked: false,
      height: DEFAULT_TRACK_HEIGHTS[trackType],
    };
    setProject((prev) => ({ ...prev, tracks: [...prev.tracks, newTrack] }));
  }, [project.tracks]);

  const handleAddElement = useCallback((trackId: string) => {
    const track = project.tracks.find((t) => t.id === trackId);
    if (!track) return;

    const newElement: TimelineElement = {
      id: `element_${Date.now()}`,
      name: `示例 ${TRACK_TYPE_LABELS[track.trackType]}`,
      trackType: track.trackType,
      startTime: 0,
      duration: 30,
      content: '示例内容',
      selected: false,
      color: TRACK_COLORS[track.trackType],
    };

    setProject((prev) => ({
      ...prev,
      tracks: prev.tracks.map((t) =>
        t.id === trackId ? { ...t, elements: [...t.elements, newElement] } : t
      ),
    }));
  }, [project.tracks]);

  const handleElementClick = useCallback((trackId: string, elementId: string) => {
    setSelectedTrackId(trackId);
    setSelectedElementId(elementId);
    setProject((prev) => ({
      ...prev,
      tracks: prev.tracks.map((t) => ({
        ...t,
        elements: t.elements.map((e) => ({
          ...e,
          selected: t.id === trackId && e.id === elementId,
        })),
      })),
    }));
  }, []);

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const rulerMarkers = [];
  for (let i = 0; i <= project.totalDuration; i += 10) {
    rulerMarkers.push(i);
  }

  return (
    <div className="timeline-footer">
      <div className="timeline-controls">
        <button className="play-btn" onClick={handlePlayPause}>
          {project.isPlaying ? (
            <Pause size={14} fill="currentColor" strokeWidth={2.2} />
          ) : (
            <Play size={14} fill="currentColor" strokeWidth={2.2} />
          )}
        </button>
        <span className="time-display">
          {formatTime(project.currentTime)} / {formatTime(project.totalDuration)}
        </span>

        <div className="zoom-control">
          <label>缩放</label>
          <input
            type="range"
            min="5"
            max="100"
            value={project.zoomLevel}
            onChange={handleZoomChange}
          />
          <span>{project.zoomLevel}%</span>
        </div>

        <div className="add-track-buttons">
          <button onClick={() => handleAddTrack(TrackType.Video)}>+ 视频</button>
          <button onClick={() => handleAddTrack(TrackType.Audio)}>+ 音频</button>
          <button onClick={() => handleAddTrack(TrackType.Document)}>+ 文档</button>
          <button onClick={() => handleAddTrack(TrackType.Subtitle)}>+ 字幕</button>
          <button onClick={() => handleAddTrack(TrackType.TTS)}>+ TTS</button>
        </div>
      </div>

      <div className="timeline-content">
        <div className="timeline-ruler">
          {rulerMarkers.map((time) => (
            <div
              key={time}
              className="ruler-marker"
              style={{ left: `${(time / project.totalDuration) * 100}%` }}
            >
              <span className="marker-label">{formatTime(time)}</span>
            </div>
          ))}
          <div
            className="playhead"
            style={{ left: `${(project.currentTime / project.totalDuration) * 100}%` }}
          />
        </div>

        <div className="timeline-tracks">
          {project.tracks.map((track) => (
            <div
              key={track.id}
              className={`timeline-track ${selectedTrackId === track.id ? 'selected' : ''}`}
              style={{ height: `${track.height}px` }}
            >
              <div className="track-header">
                <span className="track-name">{track.name}</span>
                <div className="track-controls">
                  <button
                    className={`mute-btn ${track.muted ? 'muted' : ''}`}
                    title={track.muted ? '取消静音' : '静音'}
                    onClick={() =>
                      setProject((prev) => ({
                        ...prev,
                        tracks: prev.tracks.map((t) =>
                          t.id === track.id ? { ...t, muted: !t.muted } : t
                        ),
                      }))
                    }
                  >
                    {track.muted ? <VolumeX size={13} /> : <Volume2 size={13} />}
                  </button>
                  <button
                    className={`lock-btn ${track.locked ? 'locked' : ''}`}
                    title={track.locked ? '解锁轨道' : '锁定轨道'}
                    onClick={() =>
                      setProject((prev) => ({
                        ...prev,
                        tracks: prev.tracks.map((t) =>
                          t.id === track.id ? { ...t, locked: !t.locked } : t
                        ),
                      }))
                    }
                  >
                    {track.locked ? <Lock size={13} /> : <Unlock size={13} />}
                  </button>
                  <button
                    className="add-element-btn"
                    title="添加片段"
                    onClick={() => handleAddElement(track.id)}
                  >
                    <Plus size={13} />
                  </button>
                </div>
              </div>

              <div className="track-elements">
                {track.elements.map((element) => (
                  <div
                    key={element.id}
                    className={`timeline-element ${selectedElementId === element.id ? 'selected' : ''}`}
                    style={{
                      left: `${(element.startTime / project.totalDuration) * 100}%`,
                      width: `${(element.duration / project.totalDuration) * 100}%`,
                      backgroundColor: element.color,
                    }}
                    onClick={() => handleElementClick(track.id, element.id)}
                  >
                    <span className="element-name">{element.name}</span>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
