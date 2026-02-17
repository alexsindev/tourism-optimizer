import React, { useEffect } from "react";
import {
  MapContainer,
  TileLayer,
  Marker,
  Popup,
  Polyline,
  useMap,
  useMapEvents,
} from "react-leaflet";
import L from "leaflet";
import type { Itinerary, SolveParams } from "../types";
import { getDayColor, getCategoryIcon, formatTime } from "../utils/colors";

// Fix Leaflet default marker icons
delete (L.Icon.Default.prototype as any)._getIconUrl;
L.Icon.Default.mergeOptions({
  iconRetinaUrl:
    "https://unpkg.com/leaflet@1.9.4/dist/images/marker-icon-2x.png",
  iconUrl: "https://unpkg.com/leaflet@1.9.4/dist/images/marker-icon.png",
  shadowUrl: "https://unpkg.com/leaflet@1.9.4/dist/images/marker-shadow.png",
});

interface MapViewProps {
  itinerary: Itinerary | null;
  params: SolveParams;
  onStartPointChange?: (lat: number, lng: number) => void;
  onEndPointChange?: (lat: number, lng: number) => void;
  pinDropMode?: "start" | "end" | null;
}

const MapUpdater: React.FC<{
  itinerary: Itinerary | null;
  params: SolveParams;
}> = ({ itinerary, params }) => {
  const map = useMap();

  useEffect(() => {
    if (itinerary && itinerary.days.length > 0) {
      map.setView([params.hotel_lat, params.hotel_lng], 13);
    }
  }, [itinerary, params, map]);

  return null;
};

const MapClickHandler: React.FC<{
  pinDropMode: "start" | "end" | null;
  onStartPointChange?: (lat: number, lng: number) => void;
  onEndPointChange?: (lat: number, lng: number) => void;
}> = ({ pinDropMode, onStartPointChange, onEndPointChange }) => {
  useMapEvents({
    click: (e) => {
      if (pinDropMode === "start" && onStartPointChange) {
        onStartPointChange(e.latlng.lat, e.latlng.lng);
      } else if (pinDropMode === "end" && onEndPointChange) {
        onEndPointChange(e.latlng.lat, e.latlng.lng);
      }
    },
  });
  return null;
};

const MapView: React.FC<MapViewProps> = ({
  itinerary,
  params,
  onStartPointChange,
  onEndPointChange,
  pinDropMode,
}) => {
  const hotelIcon = new L.Icon({
    iconUrl:
      "https://raw.githubusercontent.com/pointhi/leaflet-color-markers/master/img/marker-icon-2x-gold.png",
    shadowUrl: "https://unpkg.com/leaflet@1.9.4/dist/images/marker-shadow.png",
    iconSize: [25, 41],
    iconAnchor: [12, 41],
    popupAnchor: [1, -34],
    shadowSize: [41, 41],
  });

  const endIcon = new L.Icon({
    iconUrl:
      "https://raw.githubusercontent.com/pointhi/leaflet-color-markers/master/img/marker-icon-2x-red.png",
    shadowUrl: "https://unpkg.com/leaflet@1.9.4/dist/images/marker-shadow.png",
    iconSize: [25, 41],
    iconAnchor: [12, 41],
    popupAnchor: [1, -34],
    shadowSize: [41, 41],
  });

  const createNumberedIcon = (number: number, color: string) => {
    return L.divIcon({
      className: "custom-div-icon",
      html: `<div style="background-color: ${color}; width: 30px; height: 30px; border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; border: 2px solid white; box-shadow: 0 2px 5px rgba(0,0,0,0.3);">${number}</div>`,
      iconSize: [30, 30],
      iconAnchor: [15, 15],
    });
  };

  return (
    <MapContainer
      center={[params.hotel_lat, params.hotel_lng]}
      zoom={13}
      style={{ height: "100%", width: "100%" }}
    >
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />

      <Marker position={[params.hotel_lat, params.hotel_lng]} icon={hotelIcon}>
        <Popup>
          <strong>🏨 Start Point</strong>
          <br />
          {pinDropMode === "start"
            ? "Click map to move"
            : "Starting point for each day"}
        </Popup>
      </Marker>

      {/* End point marker (if different from start) */}
      {params.end_lat && params.end_lng && (
        <Marker position={[params.end_lat, params.end_lng]} icon={endIcon}>
          <Popup>
            <strong>🏁 End Point</strong>
            <br />
            {pinDropMode === "end"
              ? "Click map to move"
              : "Ending point for each day"}
          </Popup>
        </Marker>
      )}

      <MapClickHandler
        pinDropMode={pinDropMode}
        onStartPointChange={onStartPointChange}
        onEndPointChange={onEndPointChange}
      />

      {itinerary?.days.map((day) => {
        const color = getDayColor(day.day);

        return (
          <React.Fragment key={day.day}>
            {day.visits.map((visit, idx) => (
              <Marker
                key={visit.attraction_id}
                position={[
                  params.hotel_lat + (idx + 1) * 0.01,
                  params.hotel_lng + (idx + 1) * 0.01,
                ]}
                icon={createNumberedIcon(idx + 1, color)}
              >
                <Popup>
                  <div className="p-2">
                    <div className="font-bold text-lg">
                      {getCategoryIcon(visit.category)} {visit.attraction_name}
                    </div>
                    <div className="text-sm text-gray-600 mt-1">
                      <div>📅 Day {day.day}</div>
                      <div>
                        🕐 {formatTime(visit.arrival_time)} -{" "}
                        {formatTime(visit.departure_time)}
                      </div>
                      <div>💰 ${visit.fee.toFixed(2)}</div>
                      <div>⭐ Preference: {visit.preference.toFixed(2)}</div>
                      <div>🏷️ {visit.category}</div>
                    </div>
                  </div>
                </Popup>
              </Marker>
            ))}
          </React.Fragment>
        );
      })}

      <MapUpdater itinerary={itinerary} params={params} />
    </MapContainer>
  );
};

export default MapView;
