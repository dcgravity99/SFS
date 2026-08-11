/* ==========================================================================
   Siragugal Film Studio - Master Data Repository
   Architect: AG (Chief Software Architect)
   ========================================================================== */

const STUDIO_DATA = {
    // ----------------------------------------------------------------------
    // 1. Filmography & Showcase Portfolio
    // ----------------------------------------------------------------------
    films: [
        {
            id: "f1",
            title: "Wings of Destiny (Siragugal)",
            category: "Feature Film",
            year: "2026",
            director: "R. Varma",
            genre: "Epic Action / Drama",
            poster: "assets/images/poster_wings.jpg",
            badge: "NATIONAL AWARD WINNER",
            description: "A visually stunning blockbuster following an ambitious aviator navigating political intrigue and high-octane aerial battles.",
            cast: ["Vikramaditya", "Samantha Ruth", "Prakash Raj"],
            trailerUrl: "https://www.youtube.com/embed/dQw4w9WgXcQ",
            awards: ["Best Visual Effects 2025", "Best Cinematography - National Film Awards"]
        },
        {
            id: "f2",
            title: "Echoes of the Canopy",
            category: "Feature Film",
            year: "2025",
            director: "Ananya Sundaram",
            genre: "Sci-Fi / Thriller",
            poster: "assets/images/soundstage_a.jpg",
            badge: "CANNES SELECTION",
            description: "Shot entirely on Stage A's virtual production volume wall, depicting a futuristic rainforest research facility on a distant moon.",
            cast: ["Madhavan", "Sobhita Dhulipala"],
            trailerUrl: "https://www.youtube.com/embed/dQw4w9WgXcQ",
            awards: ["Cannes Technical Grand Prix", "SIIMA Best VFX"]
        },
        {
            id: "f3",
            title: "Titan Apex Chrono",
            category: "Commercial",
            year: "2026",
            director: "Karthik Subbaraj",
            genre: "Luxury Commercial",
            poster: "assets/images/hero_banner.jpg",
            badge: "ADWEEK BEST AD 2026",
            description: "A high-fashion TV commercial with high-speed Phantom Flex 4K camera capture and customized water splash rigs.",
            cast: ["Ranveer Singh"],
            trailerUrl: "https://www.youtube.com/embed/dQw4w9WgXcQ",
            awards: ["Golden Lion Commercial Award"]
        },
        {
            id: "f4",
            title: "The Last Melody of Tanjore",
            category: "Short Film",
            year: "2025",
            director: "Mani Ratnam",
            genre: "Historical Musical",
            poster: "assets/images/poster_wings.jpg",
            badge: "OSCAR SHORTLISTED",
            description: "Recorded and mixed in Siragugal Dolby Atmos Mastering Studio, featuring a 60-piece traditional orchestral arrangement.",
            cast: ["AR Rahman Ensemble"],
            trailerUrl: "https://www.youtube.com/embed/dQw4w9WgXcQ",
            awards: ["Best Sound Design - International Short Film Fest"]
        },
        {
            id: "f5",
            title: "Project Zero: Cyber Exodus",
            category: "VFX & Post",
            year: "2026",
            director: "S.S. Rajamouli Studio",
            genre: "Cyberpunk Action",
            poster: "assets/images/soundstage_a.jpg",
            badge: "VFX BLOCKBUSTER",
            description: "Over 1,200 CGI shots delivered by Siragugal VFX division featuring photorealistic creature animation and dynamic explosion simulation.",
            cast: ["Prabhas", "Rana Daggubati"],
            trailerUrl: "https://www.youtube.com/embed/dQw4w9WgXcQ",
            awards: ["Global VFX Award 2026"]
        }
    ],

    // ----------------------------------------------------------------------
    // 2. Sound Stages & Studio Facilities
    // ----------------------------------------------------------------------
    stages: [
        {
            id: "stage-a",
            type: "stage",
            name: "Sound Stage A - Virtual Production Volume",
            price: 2500,
            image: "assets/images/soundstage_a.jpg",
            specs: [
                "25,000 sq. ft. Soundproof Studio Floor (NC-25 Rated)",
                "270° Curved LED Volume Wall (ROE Visual 2.3mm Pixel Pitch)",
                "Unreal Engine 5.4 Live Camera Tracking System",
                "Silent 2000A Studio Power Grid with Backup Generators",
                "Attached Air-Conditioned Green Room & Executive Holding Suite"
            ]
        },
        {
            id: "stage-b",
            type: "stage",
            name: "Sound Stage B - Chroma Key & Water Tank",
            price: 1800,
            image: "assets/images/hero_banner.jpg",
            specs: [
                "18,000 sq. ft. Endless Cyclorama Green Screen",
                "Built-in 20ft Deep Underwater Shooting Tank with Viewing Glass",
                "35ft Ceiling Clearance with Motorized Overhead Lighting Grid",
                "40-Ton Heavy Vehicle Access Doors for On-Set Car Stunts"
            ]
        },
        {
            id: "suite-atmos",
            type: "post",
            name: "Dolby Atmos Premier Dubbing & Mix Suite",
            price: 1200,
            image: "assets/images/poster_wings.jpg",
            specs: [
                "Dolby Atmos Home & Theatrical Certification (9.1.6 Meyer Sound)",
                "Avid S6 Dual-Operator Control Surface with Pro Tools HDX3",
                "4K Laser Projection onto Micro-Perforated Acoustic Screen",
                "Acoustically Isolated ADR Voice Recording Booth"
            ]
        },
        {
            id: "suite-color",
            type: "post",
            name: "Baselight DI Color Grading & Mastering Suite",
            price: 1500,
            image: "assets/images/soundstage_a.jpg",
            specs: [
                "FilmLight Baselight FIVE Color System with Blackboard Control Panel",
                "Sony BVM-HX310 31-inch Master Monitor (1,000 nits HDR)",
                "Real-time 8K RAW Playback & Dolby Vision Metadata Mastering",
                "High-Speed Fiber SAN Infrastructure (100Gbps Direct Link)"
            ]
        }
    ],

    // ----------------------------------------------------------------------
    // 3. Camera, Lighting & Audio Rental Gear
    // ----------------------------------------------------------------------
    gear: [
        {
            id: "gear-arri-35",
            type: "gear",
            name: "ARRI Alexa 35 Camera Package",
            price: 850,
            category: "Camera",
            image: "assets/images/hero_banner.jpg",
            specs: [
                "4.6K Super 35 Sensor with 17 Stops Dynamic Range",
                "ARRI LPL / PL Mount with LDS-2 Data Support",
                "Includes Codex Compact Drives (2TB x 4) & ARRI PCA Accessories"
            ]
        },
        {
            id: "gear-red-raptor",
            type: "gear",
            name: "RED V-Raptor XL 8K VV Package",
            price: 750,
            category: "Camera",
            image: "assets/images/soundstage_a.jpg",
            specs: [
                "8K Large Format Sensor up to 120 fps at 8K 17:9",
                "Integrated Electronic ND Filters & Wireless Video Transmission",
                "Includes RED PRO CFexpress 4TB Cards & V-Lock Power Modules"
            ]
        },
        {
            id: "gear-cooke-lenses",
            type: "gear",
            name: "Cooke Anamorphic/i Full Frame Prime Lens Set",
            price: 1200,
            category: "Lenses",
            image: "assets/images/poster_wings.jpg",
            specs: [
                "5 Lens Set: 32mm, 40mm, 50mm, 75mm, 100mm (T2.3)",
                "Classic Cooke Oval Bokeh & Organic Skin Tone Rendering",
                "Includes Heavy-Duty Flight Cases & Wireless Lens Motors"
            ]
        },
        {
            id: "gear-aputure-storm",
            type: "gear",
            name: "Aputure Electro Storm CS15 LED Lighting Rig",
            price: 400,
            category: "Lighting",
            image: "assets/images/hero_banner.jpg",
            specs: [
                "1500W High-Output RGBACL Full-Color Point-Source Engine",
                "IP65 Weather-Resistant Construction for Heavy Rain Shoots",
                "Motorized Yoke with Wireless CRMX & DMX Console Control"
            ]
        }
    ],

    // ----------------------------------------------------------------------
    // 4. Client Screening Projects & Timestamped Notes
    // ----------------------------------------------------------------------
    clientProjects: {
        p1: {
            title: "Wings of Destiny (2026) - Reel 2 Color & Sound Cut",
            videoUrl: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/TearsOfSteel.mp4",
            notes: [
                { id: "n1", author: "R. Varma (Director)", timecode: "00:01:14", text: "Warm up color grade on hero facial close-up. Enhance golden backlight flare.", status: "Pending" },
                { id: "n2", author: "S. Thaman (Music Director)", timecode: "00:02:45", text: "Increase sub-bass punch on jet engine explosion in Atmos side surrounds.", status: "Resolved" },
                { id: "n3", author: "K. Reddy (Producer)", timecode: "00:04:10", text: "Clean up minor wire rig reflection on canopy glass. Outstanding VFX!", status: "Resolved" }
            ]
        },
        p2: {
            title: "Titan Commercial - Final VFX Pass",
            videoUrl: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/Sintel.mp4",
            notes: [
                { id: "n10", author: "Karthik Subbaraj", timecode: "00:00:15", text: "Add motion blur to falling watch components.", status: "Pending" }
            ]
        },
        p3: {
            title: "Echoes of Silence - Atmos Mix Approval",
            videoUrl: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            notes: [
                { id: "n20", author: "Ananya Sundaram", timecode: "00:03:00", text: "Perfect sound balance. Ready for DCI package creation.", status: "Approved" }
            ]
        }
    },

    // ----------------------------------------------------------------------
    // 5. Operations & Admin Schedule Data
    // ----------------------------------------------------------------------
    adminSchedule: [
        { id: "SFS-901", title: "Wings of Destiny (Sequel)", stage: "Stage A (LED Volume)", dates: "Aug 05 - Aug 18", gear: "ARRI Alexa 35 + Cooke Set", status: "Active" },
        { id: "SFS-902", title: "Nike Global Campaign", stage: "Stage B (Chroma Key)", dates: "Aug 08 - Aug 10", gear: "RED V-Raptor + Aputure Rig", status: "Pending" },
        { id: "SFS-903", title: "The Tanjore Symphony", stage: "Dolby Atmos Mix Suite", dates: "Aug 04 - Aug 06", gear: "Pro Tools HDX S6 Console", status: "Active" },
        { id: "SFS-904", title: "Cyber Exodus (VFX Pass)", stage: "Baselight Color Suite", dates: "Aug 12 - Aug 20", gear: "Baselight FIVE Workstation", status: "Confirmed" }
    ]
};
