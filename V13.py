import customtkinter as ct
import re
import os
from pygame import mixer

# Initialize Audio
mixer.init()

# Set global appearance
ct.set_appearance_mode("light") 

class PolishLearningApp(ct.CTk):
    def __init__(self, data_path):
        super().__init__()
        self.title("V13")
        self.geometry("900x900")
        
        # Main window background
        self.configure(fg_color="white")

        self.all_words = self.parse_ron(data_path)
        self.filtered_words = self.all_words
        self.per_page = 10
        self.current_page = 0

        self.grid_columnconfigure(0, weight=1)
        self.grid_rowconfigure(2, weight=1)

        self.setup_ui()
        self.display_page()

    def parse_ron(self, path):
        if not os.path.exists(path):
            return []
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
        entries = re.findall(r'\((.*?)\),?\s*(?=\(|$)', content, re.DOTALL)
        parsed_data = []
        keys = ['polish', 'english', 'mnemonic_combo', 'mnemonic_sentence', 'audio_path']
        for entry in entries:
            data = {}
            for key in keys:
                match = re.search(fr'{key}:\s*"(.*?)"', entry, re.DOTALL)
                if match:
                    data[key] = match.group(1).replace('\n', ' ').strip()
                else:
                    data[key] = ""
            if data['polish']:
                parsed_data.append(data)
        return parsed_data

    def setup_ui(self):
        crimson = "#DC143C"
        polish_gold = "#D4AF37" # Gold of the Polish Eagle

        # 1. Search Bar
        search_frame = ct.CTkFrame(self, fg_color="white")
        search_frame.grid(row=0, column=0, pady=(20, 10), padx=20, sticky="ew")

        ct.CTkLabel(search_frame, text="Search:", text_color=crimson).pack(side="left", padx=10)
        self.search_var = ct.StringVar()
        self.search_var.trace_add("write", self.on_search)
        
        self.search_entry = ct.CTkEntry(
            search_frame, 
            textvariable=self.search_var, 
            placeholder_text="Type Polish or English...",
            fg_color="white",
            text_color=crimson,
            border_color=crimson
        )
        self.search_entry.pack(side="left", fill="x", expand=True, padx=10, pady=10)

        # 2. Navigation Bar
        nav_frame = ct.CTkFrame(self, fg_color="white")
        nav_frame.grid(row=1, column=0, pady=10, padx=20, sticky="ew")

        self.prev_btn = ct.CTkButton(nav_frame, text="< Prev", width=80, command=self.prev_page, 
                                     fg_color=crimson, text_color="white", hover_color=polish_gold)
        self.prev_btn.pack(side="left", padx=10)

        ct.CTkLabel(nav_frame, text="Jump to Page:", text_color=crimson).pack(side="left", padx=5)
        self.page_input = ct.CTkEntry(nav_frame, width=50, fg_color="white", text_color=crimson, border_color=crimson)
        self.page_input.pack(side="left", padx=5)

        go_btn = ct.CTkButton(nav_frame, text="Go", width=50, command=self.jump_to_page, 
                              fg_color=crimson, text_color="white", hover_color=polish_gold)
        go_btn.pack(side="left", padx=5)

        self.next_btn = ct.CTkButton(nav_frame, text="Next >", width=80, command=self.next_page, 
                                     fg_color=crimson, text_color="white", hover_color=polish_gold)
        self.next_btn.pack(side="left", padx=10)

        self.page_label = ct.CTkLabel(nav_frame, text="", text_color=crimson)
        self.page_label.pack(side="right", padx=20)

        # 3. Scrollable Word List
        self.word_frame = ct.CTkScrollableFrame(self, fg_color="white", border_color="white")
        self.word_frame.grid(row=2, column=0, sticky="nsew", padx=20, pady=10)

        # 4. Info Panel
        self.info_panel = ct.CTkFrame(self, fg_color="white", border_color=crimson, border_width=2)
        self.info_panel.grid(row=3, column=0, sticky="ew", padx=20, pady=20)

        self.combo_label = ct.CTkLabel(self.info_panel, text="Mnemonic Combo", font=("Arial", 16, "bold"), text_color=crimson)
        self.combo_label.pack(pady=(10, 0))

        self.sentence_label = ct.CTkLabel(self.info_panel, text="Sentence will appear here",
                                          wraplength=800, font=("Arial", 20, "italic"), text_color=crimson)
        self.sentence_label.pack(pady=10, padx=20)

    @property
    def total_pages(self):
        return max(1, (len(self.filtered_words) - 1) // self.per_page + 1)

    def on_search(self, *args):
        query = self.search_var.get().lower()
        self.filtered_words = [
            w for w in self.all_words
            if query in w['polish'].lower() or query in w['english'].lower()
        ]
        self.current_page = 0
        self.display_page()

    def display_page(self):
        crimson = "#DC143C"
        polish_gold = "#D4AF37"
        
        for widget in self.word_frame.winfo_children():
            widget.destroy()

        start = self.current_page * self.per_page
        end = start + self.per_page
        page_words = self.filtered_words[start:end]

        for word_data in page_words:
            card = ct.CTkFrame(self.word_frame, fg_color="white")
            card.pack(fill="x", pady=2, padx=5)

            # Updated Word Button: Removed border and added Gold Hover
            btn = ct.CTkButton(
                card,
                text=f"{word_data['polish']} — {word_data['english']}",
                height=40,
                fg_color="white",
                text_color=crimson,
                border_width=0,        # Border removed
                hover_color=polish_gold, # Polish Eagle Gold
                command=lambda w=word_data: self.handle_word_click(w)
            )
            btn.pack(side="left", fill="x", expand=True, padx=10, pady=5)

        self.page_label.configure(text=f"Page {self.current_page + 1} of {self.total_pages}")
        self.prev_btn.configure(state="normal" if self.current_page > 0 else "disabled")
        self.next_btn.configure(state="normal" if self.current_page < self.total_pages - 1 else "disabled")

    def handle_word_click(self, data):
        path = data.get('audio_path', "")
        if path and os.path.exists(path):
            try:
                mixer.music.load(path)
                mixer.music.play()
            except: pass

        self.combo_label.configure(text=f"Combo: {data.get('mnemonic_combo', 'N/A')}")
        self.sentence_label.configure(text=data.get('mnemonic_sentence', 'No sentence available.'))

    def next_page(self):
        if self.current_page < self.total_pages - 1:
            self.current_page += 1
            self.display_page()

    def prev_page(self):
        if self.current_page > 0:
            self.current_page -= 1
            self.display_page()

    def jump_to_page(self):
        try:
            p = int(self.page_input.get()) - 1
            if 0 <= p < self.total_pages:
                self.current_page = p
                self.display_page()
        except: pass

if __name__ == "__main__":
    app = PolishLearningApp("words.ron")
    app.mainloop()
