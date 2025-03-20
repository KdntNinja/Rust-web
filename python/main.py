import json

class Quiz:
    def __init__(self):
        self.score: int = 0
        self.user_answers: dict = {}
        self.questions: dict = {
            "Which country is the city of Berlin in?": "germany",
            "Which country is the city of Newcastle in?": "uk",
            "Which country is the city of Marseille in?": "france",
            "Which country is the city of Rome in?": "italy",
            "Which country is the cirty of Brussles in?": "belgium"
        }

    def ask_questions(self):
        for question, answer in self.questions.items():
            response = input(question + " ")
            self.user_answers[question] = {
                "user_answer": response.lower(),
                "correct_answer": answer
            }

    def save_answers_to_file(self):
        with open("answers.json", "w") as f:
            json.dump(self.user_answers, f, indent=4)

    def load_answers_from_file(self):
        with open("answers.json", "r") as f:
            return json.load(f)

    def calculate_score(self, loaded_answers):
        self.score = sum(
            1 for question, data in loaded_answers.items()
            if data["user_answer"] == data["correct_answer"]
        )

    def display_score(self):
        print(f"Your score is {self.score} out of {len(self.questions)}")

    def run(self):
        self.ask_questions()
        self.save_answers_to_file()
        loaded_answers = self.load_answers_from_file()
        self.calculate_score(loaded_answers)
        self.display_score()


if __name__ == "__main__":
    quiz = Quiz()
    quiz.run()