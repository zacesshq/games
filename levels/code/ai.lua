-- Learn to Code: the AI in Python track. Each level teaches one idea with a short lesson and a few
-- lines of code with blanks marked {1}, {2} and so on. A blank lists the answers that work,
-- a hint and the mistakes people often make, each with a message that explains it. The output
-- is what the finished code prints or does.
-- The ideas of artificial intelligence and robotics, written in plain Python.
-- Answers are compared inside the game; nothing typed is ever run.

return {
  language = [==[AI in Python]==],
  id = [==[ai]==],
  quotes_equal = true,
  levels = {
    {
      name = [==[Agents]==], topic = [==[Agents]==],
      lesson = { [==[An agent senses its surroundings and acts. A simple reflex agent maps what it senses straight to an action.]==] },
      code = { [==[def act(sensor):]==], [==[    if sensor == "obstacle":]==], [==[        return {1}]==], [==[    return "forward"]==] },
      blanks = {
        { answers = { [==["turn"]==] }, hint = [==[What a robot should do when something is in the way, as a string.]==], wrong = { { [==[turn]==], [==[The actions are strings, so turn needs quotes.]==] } } },
      },
      output = { [==[act("obstacle") is "turn"]==] },
    },
    {
      name = [==[Search as a problem]==], topic = [==[Search]==],
      lesson = { [==[Search needs a start state, a goal test and a way to get the next states, called successors.]==] },
      code = { [==[def is_goal(state):]==], [==[    return state {1} goal]==] },
      blanks = {
        { answers = { [==[==]==] }, hint = [==[The comparison for equal.]==], wrong = { { [==[=]==], [==[One = assigns. To compare use ==.]==] } } },
      },
      output = { [==[the search stops when the goal is reached]==] },
    },
    {
      name = [==[Breadth first]==], topic = [==[Search]==],
      lesson = { [==[Breadth-first search expands the shallowest state first, so it finds the path with the fewest steps.]==] },
      code = { [==[from collections import deque]==], [==[frontier = deque([start])]==], [==[state = frontier.{1}()]==] },
      blanks = {
        { answers = { [==[popleft]==] }, hint = [==[Take from the left, the oldest end.]==], wrong = { { [==[pop]==], [==[pop takes the newest item, which gives depth-first search.]==] } } },
      },
      output = { [==[the oldest state is expanded first]==] },
    },
    {
      name = [==[Depth first]==], topic = [==[Search]==],
      lesson = { [==[Depth-first search expands the newest state first, following one path deep before trying others.]==] },
      code = { [==[frontier = [start]]==], [==[state = frontier.{1}()]==] },
      blanks = {
        { answers = { [==[pop]==] }, hint = [==[Take the last item added.]==], wrong = { { [==[pop(0)]==], [==[pop(0) takes the oldest, which is breadth-first.]==] } } },
      },
      output = { [==[the newest state is expanded first]==] },
    },
    {
      name = [==[Not going round in circles]==], topic = [==[Search]==],
      lesson = { [==[Keeping a set of visited states stops a search revisiting them.]==] },
      code = { [==[visited = {1}()]==], [==[if state not in visited:]==], [==[    visited.add(state)]==] },
      blanks = {
        { answers = { [==[set]==] }, hint = [==[A collection with fast membership tests.]==], wrong = { { [==[list]==], [==[A list works but x in list checks every item. A set is O(1).]==] } } },
      },
      output = { [==[each state is expanded once]==] },
    },
    {
      name = [==[A star]==], topic = [==[Search]==],
      lesson = { [==[A* expands the state with the lowest f = g + h: the cost so far plus a guess of the cost to go.]==] },
      code = { [==[f = g {1} h]==] },
      blanks = {
        { answers = { [==[+]==] }, hint = [==[Combine the two costs.]==], wrong = { { [==[*]==], [==[A* adds the cost so far and the estimate.]==] } } },
      },
      output = { [==[the most promising state is expanded first]==] },
    },
    {
      name = [==[A good guess]==], topic = [==[Search]==],
      lesson = { [==[On a grid with four moves, Manhattan distance never overestimates, so A* with it finds a shortest path.]==] },
      code = { [==[def h(a, b):]==], [==[    return {1}(a[0] - b[0]) + abs(a[1] - b[1])]==] },
      blanks = {
        { answers = { [==[abs]==] }, hint = [==[Distance cannot be negative.]==], wrong = { { [==[sqrt]==], [==[Straight-line distance is fine too, but this formula adds absolute differences.]==] } } },
      },
      output = { [==[h((0, 0), (3, 4)) is 7]==] },
    },
    {
      name = [==[Facts and rules]==], topic = [==[Knowledge representation]==],
      lesson = { [==[Knowledge can be stored as facts and if-then rules. Forward chaining applies rules to known facts to find new ones.]==] },
      code = { [==[facts = {"raining"}]==], [==[if "raining" in facts:]==], [==[    facts.{1}("wet ground")]==] },
      blanks = {
        { answers = { [==[add]==] }, hint = [==[The set method that puts a new fact in.]==], wrong = { { [==[append]==], [==[facts is a set. Sets use add.]==] } } },
      },
      output = { [==[facts now holds raining and wet ground]==] },
    },
    {
      name = [==[Fitting a line]==], topic = [==[Machine learning: regression]==],
      lesson = { [==[Regression predicts a number. A straight line y = w * x + b is the simplest model.]==] },
      code = { [==[w, b = 2.0, 1.0]==], [==[def predict(x):]==], [==[    return w * x {1} b]==] },
      blanks = {
        { answers = { [==[+]==] }, hint = [==[Add the intercept.]==], wrong = { { [==[*]==], [==[The intercept is added: w * x + b.]==] } } },
      },
      output = { [==[predict(3) is 7.0]==] },
    },
    {
      name = [==[How wrong is it?]==], topic = [==[Machine learning: regression]==],
      lesson = { [==[Mean squared error averages the squared gaps between predictions and true values.]==] },
      code = { [==[mse = sum((p - y) {1} 2 for p, y in zip(preds, ys)) / len(ys)]==] },
      blanks = {
        { answers = { [==[**]==] }, hint = [==[Python's power operator.]==], wrong = { { [==[^]==], [==[^ is bitwise xor in Python. Powers use **.]==] } } },
      },
      output = { [==[0 when every prediction is right]==] },
    },
    {
      name = [==[Yes or no]==], topic = [==[Machine learning: classification]==],
      lesson = { [==[Classification predicts a label. A threshold turns a score into a yes or no.]==] },
      code = { [==[def label(score):]==], [==[    return "spam" if score {1} 0.5 else "ham"]==] },
      blanks = {
        { answers = { [==[>]==], [==[>=]==] }, hint = [==[Above the threshold means spam.]==], wrong = { { [==[<]==], [==[That would mark the low scores as spam.]==] } } },
      },
      output = { [==[label(0.9) is "spam"]==] },
    },
    {
      name = [==[Nearest neighbours]==], topic = [==[Machine learning: k-NN]==],
      lesson = { [==[k-NN labels a new point by the majority label of the k training points closest to it.]==] },
      code = { [==[nearest = sorted(points, key=lambda p: distance(p, new))[:{1}]]==] },
      blanks = {
        { answers = { [==[k]==] }, hint = [==[How many neighbours vote.]==], wrong = { { [==[1]==], [==[That fixes k at one. Use the variable k.]==] } } },
      },
      output = { [==[the k closest points vote]==] },
    },
    {
      name = [==[Asking questions]==], topic = [==[Machine learning: decision trees]==],
      lesson = { [==[A decision tree asks a question at each node and follows a branch until a leaf gives the answer.]==] },
      code = { [==[def classify(volts):]==], [==[    if volts > 3.3:]==], [==[        return "needs a resistor"]==], [==[    {1}:]==], [==[        return "safe"]==] },
      blanks = {
        { answers = { [==[else]==] }, hint = [==[The branch for every other case.]==], wrong = { { [==[elif]==], [==[elif needs its own test. Here the leftover case is else.]==] } } },
      },
      output = { [==[classify(5) is "needs a resistor"]==] },
    },
    {
      name = [==[Fair testing]==], topic = [==[Machine learning: evaluation]==],
      lesson = { [==[A model is judged on data it has not seen. Keep a test set apart from the training set.]==] },
      code = { [==[split = int(0.8 * len(data))]==], [==[train, test = data[:split], data[{1}:]]==] },
      blanks = {
        { answers = { [==[split]==] }, hint = [==[Start where the training data stops.]==], wrong = { { [==[0]==], [==[data[0:] would test on the training data too.]==] } } },
      },
      output = { [==[80% to train, 20% to test]==] },
    },
    {
      name = [==[Accuracy]==], topic = [==[Machine learning: evaluation]==],
      lesson = { [==[Accuracy is the share of predictions that are right.]==] },
      code = { [==[accuracy = sum(p == y for p, y in zip(preds, ys)) / {1}(ys)]==] },
      blanks = {
        { answers = { [==[len]==] }, hint = [==[Divide by how many there are.]==], wrong = { { [==[sum]==], [==[sum would add the labels. Divide by len(ys).]==] } } },
      },
      output = { [==[0.9 means nine in ten are right]==] },
    },
    {
      name = [==[Seeing and moving]==], topic = [==[Robotics]==],
      lesson = { [==[A robot runs a loop: sense, decide, act. A proportional controller turns more the further it is off course.]==] },
      code = { [==[error = target_heading - heading]==], [==[turn = {1} * error]==] },
      blanks = {
        { answers = { [==[kp]==], [==[Kp]==], [==[k]==] }, hint = [==[The gain, often named kp.]==], wrong = { { [==[error]==], [==[error * error would grow far too fast. Multiply by a gain such as kp.]==] } } },
      },
      output = { [==[small errors give gentle turns]==] },
    },
  },
}
