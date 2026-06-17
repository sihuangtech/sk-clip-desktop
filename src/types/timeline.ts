export enum TrackType {
  Video = 'Video',
  Audio = 'Audio',
  Document = 'Document',
  Subtitle = 'Subtitle',
  TTS = 'TTS',
}

export interface TimelineElement {
  id: string;
  name: string;
  trackType: TrackType;
  startTime: number;
  duration: number;
  content: string;
  selected: boolean;
  color: string;
}

export interface TimelineTrack {
  id: string;
  name: string;
  trackType: TrackType;
  elements: TimelineElement[];
  muted: boolean;
  locked: boolean;
  height: number;
}

export interface TimelineProject {
  name: string;
  totalDuration: number;
  tracks: TimelineTrack[];
  currentTime: number;
  zoomLevel: number;
  isPlaying: boolean;
}

export const DEFAULT_TRACK_HEIGHTS: Record<TrackType, number> = {
  [TrackType.Video]: 80,
  [TrackType.Audio]: 60,
  [TrackType.Document]: 60,
  [TrackType.Subtitle]: 40,
  [TrackType.TTS]: 60,
};

export const TRACK_COLORS: Record<TrackType, string> = {
  [TrackType.Video]: '#4CAF50',
  [TrackType.Audio]: '#2196F3',
  [TrackType.Document]: '#FF9800',
  [TrackType.Subtitle]: '#9C27B0',
  [TrackType.TTS]: '#F44336',
};

export const TRACK_TYPE_LABELS: Record<TrackType, string> = {
  [TrackType.Video]: '视频轨道',
  [TrackType.Audio]: '音频轨道',
  [TrackType.Document]: '文档轨道',
  [TrackType.Subtitle]: '字幕轨道',
  [TrackType.TTS]: 'TTS 轨道',
};
