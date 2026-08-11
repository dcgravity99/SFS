#!/usr/bin/env python3

import time


class SIRAOrchestrator:

    def __init__(self):
        self.modules = {
            "story": "Story Architect",
            "screenwriter": "Screenwriter",
            "director": "Director",
            "camera": "Cinematographer",
            "producer": "Producer",
            "editor": "Editor"
        }


    def status(self):

        print("🎬 SIRA AI ORCHESTRATOR")
        print("----------------------")

        for key,value in self.modules.items():
            print(f"✓ {key} : {value}")

        print("----------------------")
        print("All modules registered")


    def execute(self, pipeline):

        print("🎬 Pipeline Started")

        for step in pipeline:

            print(
                "Running:",
                self.modules.get(step, step)
            )

            time.sleep(1)

        print("🎬 Pipeline Completed")
