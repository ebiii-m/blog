from db.main import DataBase
from flask import Flask, request, jsonify, Response
import random
import string
import argparse

class App:
    def __init__(self, host: str, port: int, path: str) -> None:
        self._app = Flask(__name__)
        self._db = DataBase(path)
        self._host = host
        self._port = port
        self._init()

    def _init(self) -> None:
        @self._app.route('/new-rec/', methods=["POST"])
        def new_record() -> Response:
            try:
                data = request.get_json()
                if not data or 'data' not in data:
                    return jsonify({
                        "status": "error",
                        "message": "Invalid request data. 'data' field is required."
                    }), 400
                
                rec_id = self._create_param(12)
                self._db.insert(rec_id, data["data"])
                return jsonify({
                    "status": "success",
                    "rec_id": rec_id
                }), 201
            except Exception as e:
                return jsonify({
                    "status": "error",
                    "message": f"Internal server error: {str(e)}"
                }), 500

        @self._app.route('/get-rec/', methods=["GET"])
        def get_record() -> Response:
            try:
                rec_id = request.args.get("rec_id")
                if not rec_id:
                    return jsonify({
                        "status": "error",
                        "message": "rec_id parameter is required"
                    }), 400
                
                data = self._db.get(rec_id)
                if data is None:
                    return jsonify({
                        "status": "error",
                        "message": "Record not found"
                    }), 404
                
                return jsonify({
                    "status": "success",
                    "data": data
                }), 200
            except Exception as e:
                return jsonify({
                    "status": "error",
                    "message": f"Internal server error: {str(e)}"
                }), 500

        @self._app.route('/find-rec/', methods=["GET"])
        def find_record() -> Response:
            try:
                param = request.args.get("param")
                param_value = request.args.get("param_value")
                
                if not param or not param_value:
                    return jsonify({
                        "status": "error",
                        "message": "Both 'param' and 'param_value' parameters are required"
                    }), 400
                
                data = self._db.filter_users_by_param(param=param, value=param_value)
                if not data:
                    return jsonify({
                        "status": "error",
                        "message": "No records found"
                    }), 404
                
                if len(data) == 1:
                    return jsonify({
                        "status": "success",
                        "data": data[0]
                    }), 200
                else:
                    return jsonify({
                        "status": "error",
                        "message": "Multiple records found, use /find-rec-group/ instead"
                    }), 400
            except Exception as e:
                return jsonify({
                    "status": "error",
                    "message": f"Internal server error: {str(e)}"
                }), 500

        @self._app.route('/update-rec/', methods=["POST"])
        def update_record() -> Response:
            try:
                data = request.get_json()
                if not data or 'rec_id' not in data or 'data' not in data:
                    return jsonify({
                        "status": "error",
                        "message": "Invalid request data. Both 'rec_id' and 'data' fields are required."
                    }), 400
                
                rec_id = data["rec_id"]
                if not self._db.user_exists(rec_id):
                    return jsonify({
                        "status": "error",
                        "message": "Record not found"
                    }), 404
                
                self._db.update(rec_id, data["data"])
                return jsonify({
                    "status": "success",
                    "message": "Record updated successfully"
                }), 200
            except Exception as e:
                return jsonify({
                    "status": "error",
                    "message": f"Internal server error: {str(e)}"
                }), 500

        @self._app.route('/find-rec-group/', methods=["GET"])
        def find_record_group() -> Response:
            try:
                param = request.args.get("param")
                param_value = request.args.get("param_value")
                
                if not param or not param_value:
                    return jsonify({
                        "status": "error",
                        "message": "Both 'param' and 'param_value' parameters are required"
                    }), 400
                
                data = self._db.filter_users_by_param(param=param, value=param_value)
                if not data:
                    return jsonify({
                        "status": "error",
                        "message": "No records found"
                    }), 404
                
                return jsonify({
                    "status": "success",
                    "data": data
                }), 200
            except Exception as e:
                return jsonify({
                    "status": "error",
                    "message": f"Internal server error: {str(e)}"
                }), 500

    def _create_param(self, length: int) -> str:
        characters = string.ascii_letters + string.digits
        return ''.join(random.choice(characters) for _ in range(length))

    def run(self) -> None:
        self._app.run(host=self._host, port=self._port, debug=False)



def main():
    parser = argparse.ArgumentParser(description='Run Flask API for database operations')
    
    parser.add_argument('--host', type=str, default='127.0.0.1',
                       help='Host address to run the API on (default: 127.0.0.1)')
    
    parser.add_argument('--port', type=int, default=5000,
                       help='Port number to run the API on (default: 5000)')
    
    parser.add_argument('--path', type=str, required=True,
                       help='Path to the database directory (required)')
    
    args = parser.parse_args()

    
    api = App(host=args.host, port=args.port, path=args.path)
    print(f"Starting API on http://{args.host}:{args.port} with database path: {args.path}")
    api.run()

if __name__ == '__main__':
    main()